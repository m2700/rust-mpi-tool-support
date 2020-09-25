use std::{
    marker::PhantomData,
    mem::{forget, transmute, MaybeUninit},
    ops::Deref,
    os::raw::c_int,
    ptr,
};

local_mod!(
    use mpi_sys::*;
    use crate::{Error, Process, RmpiResult, Group};
);

mod allgather;
mod allreduce;
mod alltoall;
mod barrier;
mod create_subset;
mod scan;

/// non MPI_COMM_NULL Communicator handle that frees automatically
/// A Communicator cannot live longer than its rmpi context, defined by 'ctx.
/// However, a Communicator can safely be shared between threads
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug)]
pub struct Communicator<'ctx> {
    pub(crate) raw: MPI_Comm,
    context_lifetime: PhantomData<&'ctx ()>,
}
impl<'ctx> Drop for Communicator<'ctx> {
    #[inline]
    fn drop(&mut self) {
        unsafe { ptr::read(self).free().unwrap() }
    }
}
impl<'ctx> Deref for Communicator<'ctx> {
    type Target = MPI_Comm;
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
impl<'ctx> Clone for Communicator<'ctx> {
    #[inline]
    fn clone(&self) -> Self {
        self.duplicate().unwrap()
    }
}
impl<'ctx> Communicator<'ctx> {
    #[inline]
    pub unsafe fn from_raw(raw: MPI_Comm) -> Self {
        Self {
            raw,
            context_lifetime: PhantomData,
        }
    }
    #[inline]
    pub unsafe fn from_raw_ref(raw: &MPI_Comm) -> &Self {
        transmute::<&MPI_Comm, &Self>(raw)
    }
    #[inline]
    pub fn as_raw(&self) -> MPI_Comm {
        self.raw
    }
    #[inline]
    pub fn into_raw(&self) -> MPI_Group {
        let raw = self.raw;
        forget(self);
        raw
    }

    #[inline]
    pub fn get_process(&self, rank: c_int) -> Process {
        Process {
            communicator: self,
            rank,
        }
    }
    #[inline]
    pub fn current_process(&self) -> RmpiResult<Process> {
        Ok(self.get_process(self.current_rank()?))
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn duplicate_with<F>(&self, mpi_comm_dup: F) -> RmpiResult<Self>
        where
            F: FnOnce(MPI_Comm, *mut MPI_Comm) -> c_int,
        {
            let mut new_raw = MPI_COMM_NULL;
            let res = Error::from_mpi_res(mpi_comm_dup(self.raw, &mut new_raw));
            forget(self);
            res.map(|()| Self::from_raw(new_raw))
        }
    );
    #[inline]
    pub fn duplicate(&self) -> RmpiResult<Self> {
        unsafe { self.duplicate_with(|comm, newcomm| MPI_Comm_dup(comm, newcomm)) }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn size_with<F>(&self, mpi_comm_size: F) -> RmpiResult<c_int>
        where
            F: FnOnce(MPI_Comm, *mut c_int) -> c_int,
        {
            let mut size = 0;
            Error::from_mpi_res(mpi_comm_size(self.as_raw(), &mut size)).map(|()| size)
        }
    );
    #[inline]
    pub fn size(&self) -> RmpiResult<c_int> {
        unsafe { self.size_with(|comm, size| MPI_Comm_size(comm, size)) }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn current_rank_with<F>(&self, mpi_comm_rank: F) -> RmpiResult<c_int>
        where
            F: FnOnce(MPI_Comm, *mut c_int) -> c_int,
        {
            self.size_with(mpi_comm_rank)
        }
    );
    #[inline]
    pub fn current_rank(&self) -> RmpiResult<c_int> {
        unsafe { self.current_rank_with(|comm, rank| MPI_Comm_rank(comm, rank)) }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn group_with<F>(&self, mpi_comm_group: F) -> RmpiResult<Group>
        where
            F: FnOnce(MPI_Comm, *mut MPI_Group) -> c_int,
        {
            let mut group = MaybeUninit::uninit();
            Error::from_mpi_res(mpi_comm_group(self.as_raw(), group.as_mut_ptr()))
                .map(|()| Group::from_raw(group.assume_init()))
        }
    );
    #[inline]
    pub fn group(&self) -> RmpiResult<Group> {
        unsafe { self.group_with(|comm, group| MPI_Comm_group(comm, group)) }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn free_with<F>(self, mpi_comm_free: F) -> RmpiResult
        where
            F: FnOnce(*mut MPI_Comm) -> c_int,
        {
            let mut mpi_comm = *self;
            let res = Error::from_mpi_res(mpi_comm_free(&mut mpi_comm));
            forget(self);
            res
        }
    );
    #[inline]
    pub fn free(self) -> RmpiResult {
        unsafe { self.free_with(|comm| MPI_Comm_free(comm)) }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn abort_with<F>(self, mpi_abort: F, error: Error) -> RmpiResult
        where
            F: FnOnce(MPI_Comm, c_int) -> c_int,
        {
            let mpi_comm = *self;
            let res = Error::from_mpi_res(mpi_abort(mpi_comm, error.into()));
            forget(self);
            res
        }
    );
    #[inline]
    pub fn abort(self, error: Error) -> RmpiResult {
        unsafe { self.abort_with(|comm, errorcode| MPI_Abort(comm, errorcode), error) }
    }
}

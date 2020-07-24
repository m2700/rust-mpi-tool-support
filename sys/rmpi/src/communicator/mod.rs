use std::{
    mem::{forget, transmute},
    ops::Deref,
    os::raw::c_int,
    ptr,
};

local_mod!(
    use mpi_sys::*;
    use crate::{Error, Process, RmpiResult};
);

mod allgather;
mod allreduce;
mod barrier;
mod create_subset;
mod scan;

/// non MPI_COMM_NULL Communicator handle that frees automatically
#[repr(transparent)]
pub struct Communicator(pub(crate) MPI_Comm);
impl Drop for Communicator {
    #[inline]
    fn drop(&mut self) {
        match self.0 {
            MPI_COMM_NULL => unreachable!(),
            MPI_COMM_WORLD | MPI_COMM_SELF => {}
            _ => unsafe { ptr::read(self).free().unwrap() },
        }
    }
}
impl Deref for Communicator {
    type Target = MPI_Comm;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Communicator {
    #[inline]
    pub unsafe fn from_raw(raw: MPI_Comm) -> Self {
        Self(raw)
    }
    #[inline]
    pub unsafe fn from_raw_ref(raw: &MPI_Comm) -> &Self {
        transmute::<&MPI_Comm, &Self>(raw)
    }
    #[inline]
    pub fn as_raw(&self) -> MPI_Comm {
        self.0
    }
    #[inline]
    pub fn into_raw(&self) -> MPI_Group {
        let raw = self.0;
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
        unsafe { self.current_rank_with(|comm, size| MPI_Comm_rank(comm, size)) }
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
}

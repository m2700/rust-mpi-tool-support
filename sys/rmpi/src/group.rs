use std::{
    mem::{forget, transmute, MaybeUninit},
    os::raw::*,
    ptr,
};

local_mod!(
    use mpi_sys::*;
    use crate::{Error, RmpiResult};
);

#[repr(transparent)]
pub struct Group(MPI_Group);
impl Drop for Group {
    #[inline]
    fn drop(&mut self) {
        unsafe { ptr::read(self).free().unwrap() }
    }
}
impl Group {
    #[inline]
    pub unsafe fn from_raw(raw: MPI_Group) -> Self {
        Self(raw)
    }
    #[inline]
    pub unsafe fn from_raw_ref(raw: &MPI_Group) -> &Self {
        transmute::<&MPI_Group, &Self>(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> MPI_Group {
        self.0
    }
    #[inline]
    pub fn into_raw(&self) -> MPI_Group {
        let raw = self.0;
        forget(self);
        raw
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn incl_with<F>(&self, mpi_group_incl: F, ranks: &[c_int]) -> RmpiResult<Self>
        where
            F: FnOnce(MPI_Group, c_int, *const c_int, *mut MPI_Group) -> c_int,
        {
            let mut mpi_group = MaybeUninit::uninit();
            Error::from_mpi_res(mpi_group_incl(
                self.0,
                ranks.len() as c_int,
                ranks.as_ptr(),
                mpi_group.as_mut_ptr(),
            ))
            .map(|()| Self::from_raw(mpi_group.assume_init()))
        }
    );
    #[inline]
    pub unsafe fn incl(&self, ranks: &[c_int]) -> RmpiResult<Self> {
        self.incl_with(
            |group, n, ranks, newgroup| MPI_Group_incl(group, n, ranks, newgroup),
            ranks,
        )
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn free_with<F>(self, mpi_group_free: F) -> RmpiResult
        where
            F: FnOnce(*mut MPI_Comm) -> c_int,
        {
            let mut mpi_group = self.as_raw();
            let res = Error::from_mpi_res(mpi_group_free(&mut mpi_group));
            forget(self);
            res
        }
    );
    #[inline]
    pub fn free(self) -> RmpiResult {
        unsafe { self.free_with(|group| MPI_Group_free(group)) }
    }
}

use std::{mem::MaybeUninit, os::raw::*};

local_mod!(
    use mpi_sys::*;
    use crate::{Error, Group, RmpiResult};
);

use super::Communicator;

impl Communicator {
    tool_mode_item!(
        #[inline]
        pub unsafe fn create_subset_with<F>(
            &self,
            mpi_comm_create: F,
            group: &Group,
        ) -> RmpiResult<Self>
        where
            F: FnOnce(MPI_Comm, MPI_Group, *mut MPI_Comm) -> c_int,
        {
            let mut newcomm = MaybeUninit::uninit();
            Error::from_mpi_res(mpi_comm_create(
                self.as_raw(),
                group.as_raw(),
                newcomm.as_mut_ptr(),
            ))
            .map(|()| Self::from_raw(newcomm.assume_init()))
        }
    );
    #[inline]
    pub fn create_subset(&self, group: &Group) -> RmpiResult<Self> {
        unsafe {
            self.create_subset_with(
                |comm, group, newcomm| MPI_Comm_create(comm, group, newcomm),
                group,
            )
        }
    }
}

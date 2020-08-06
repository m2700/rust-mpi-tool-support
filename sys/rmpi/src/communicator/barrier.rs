use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{ Error,  RmpiResult};
);

use super::Communicator;

impl Communicator {
    tool_mode_item!(
        #[inline]
        pub unsafe fn barrier_with<F>(&self, mpi_barrier: F) -> RmpiResult
        where
            F: FnOnce(MPI_Comm) -> c_int,
        {
            Error::from_mpi_res(mpi_barrier(self.as_raw()))
        }
    );
    #[inline]
    pub fn barrier(&self) -> RmpiResult {
        unsafe { self.barrier_with(|comm| MPI_Barrier(comm)) }
    }
}

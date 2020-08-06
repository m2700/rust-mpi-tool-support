use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{BufferMut, Error, RmpiResult};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn bcast_with<F, B>(&self, mpi_bcast: F, mut buffer: B) -> RmpiResult
        where
            B: BufferMut,
            F: FnOnce(*mut c_void, c_int, MPI_Datatype, c_int, MPI_Comm) -> c_int,
        {
            let (buf, count) = buffer.as_raw_mut();
            Error::from_mpi_res(mpi_bcast(
                buf,
                count,
                buffer.item_datatype(),
                self.rank,
                self.communicator.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn bcast<B: BufferMut>(&self, buffer: B) -> RmpiResult {
        unsafe {
            self.bcast_with(
                |buf, count, datatype, rank, comm| MPI_Bcast(buf, count, datatype, rank, comm),
                buffer,
            )
        }
    }
}

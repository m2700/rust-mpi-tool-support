use std::os::raw::*;

use mpi_sys::*;

use crate::{Buffer, Error, RmpiResult, Tag};

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item! {
        #[inline]
        pub unsafe fn bcast_with<F, B>(&mut self, mpi_bcast: F, buffer: &mut B) -> RmpiResult
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *mut c_void,
                c_int,
                MPI_Datatype,
                c_int,
                MPI_Comm
            ) -> c_int {
            let (buf, count) = buffer.into_raw_mut();
            Error::from_mpi_res(
                mpi_bcast(
                    buf,
                    count,
                    buffer.item_datatype(),
                    self.rank,
                    self.communicator.as_raw(),
                )
            )
        }
    }
    #[inline]
    pub fn bcast<B: Buffer + ?Sized>(&mut self, buffer: &mut B, tag: Tag) -> RmpiResult {
        unsafe {
            self.bcast_with(
                |buf, count, datatype, rank, comm| MPI_Bcast(buf, count, datatype, rank, comm),
                buffer,
            )
        }
    }
}

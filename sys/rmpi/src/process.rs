use std::os::raw::c_int;

use mpi_sys::MPI_Send;

use crate::{Buffer, Communicator, Error, RmpiResult};

pub struct Process<'c> {
    pub(crate) communicator: &'c Communicator,
    pub(crate) rank: c_int,
}
impl<'c> Process<'c> {
    tool_mode_item! {
        #[inline]
        pub unsafe fn send_with<F, B>(&mut self, mpi_send: F, buffer: &B, tag: c_int) -> RmpiResult
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *const ::std::os::raw::c_void,
                ::std::os::raw::c_int,
                mpi_sys::MPI_Datatype,
                ::std::os::raw::c_int,
                ::std::os::raw::c_int,
                mpi_sys::MPI_Comm,
            ) -> ::std::os::raw::c_int,{
            let (buf, count) = buffer.into_raw();
            Error::from_mpi_res(
                mpi_send(
                    buf,
                    count,
                    buffer.item_datatype(),
                    self.rank,
                    tag,
                    self.communicator.as_raw(),
                )
            )
        }
    }
    #[inline]
    pub fn send<B: Buffer + ?Sized>(&mut self, buffer: &B, tag: c_int) -> RmpiResult {
        unsafe {
            self.send_with(
                |buf, count, datatype, rank, tag, comm| {
                    MPI_Send(buf, count, datatype, rank, tag, comm)
                },
                buffer,
                tag,
            )
        }
    }
}

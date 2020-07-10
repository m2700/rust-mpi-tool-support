use std::os::raw::*;

use mpi_sys::*;

use crate::{request::Request, Buffer, Error, RmpiResult, Tag};

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item! {
        #[inline]
        pub unsafe fn irecv_with<F, B>(&mut self, mpi_irecv: F, buffer: &mut B, tag: Tag) -> RmpiResult<Request>
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *mut c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
                *mut MPI_Request
            ) -> c_int,{

            let mut request = 0;
            let (buf, count) = buffer.into_raw_mut();
            let res = mpi_irecv(
                buf,
                count,
                buffer.item_datatype(),
                self.rank,
                *tag,
                self.communicator.as_raw(),
                &mut request
            );
            Error::from_mpi_res(
                res
            ).map(|()| Request::from_raw(request))
        }
    }
    #[inline]
    pub fn irecv<B: Buffer + ?Sized>(&mut self, buffer: &mut B, tag: Tag) -> RmpiResult<Request> {
        unsafe {
            self.irecv_with(
                |buf, count, datatype, rank, tag, comm, status| {
                    MPI_Irecv(buf, count, datatype, rank, tag, comm, status)
                },
                buffer,
                tag,
            )
        }
    }
}

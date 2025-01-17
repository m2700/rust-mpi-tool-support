use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{request::Request, BufferMut, Error, RmpiResult, Tag};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn irecv_with<'b, F, B: 'b>(
            &self,
            mpi_irecv: F,
            mut buffer: B,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            B: BufferMut,
            F: FnOnce(
                *mut c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
                *mut MPI_Request,
            ) -> c_int,
        {
            let mut request = 0;
            let (buf, count) = buffer.as_raw_mut();
            let res = mpi_irecv(
                buf,
                count,
                buffer.item_datatype(),
                self.rank,
                *tag,
                self.communicator.as_raw(),
                &mut request,
            );
            Error::from_mpi_res(res).map(|()| Request::from_raw(request))
        }
    );
    #[inline]
    pub fn irecv<'b, B: BufferMut + 'b>(&self, buffer: B, tag: Tag) -> RmpiResult<Request<'b>> {
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

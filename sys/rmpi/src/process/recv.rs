use std::{mem::MaybeUninit, os::raw::*};

local_mod!(
    use mpi_sys::*;
    use crate::{BufferMut, Error, RmpiResult, Status, Tag};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn recv_with<F, B>(
            &self,
            mpi_recv: F,
            mut buffer: B,
            tag: Tag,
            status_ignore: bool,
        ) -> RmpiResult<Option<Status>>
        where
            B: BufferMut,
            F: FnOnce(
                *mut c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
                *mut MPI_Status,
            ) -> c_int,
        {
            let mut status = if status_ignore {
                None
            } else {
                Some(MaybeUninit::uninit())
            };
            let (buf, count) = buffer.as_raw_mut();
            let res = mpi_recv(
                buf,
                count,
                buffer.item_datatype(),
                self.rank,
                *tag,
                self.communicator.as_raw(),
                status
                    .as_mut()
                    .map(|s| s.as_mut_ptr())
                    .unwrap_or(MPI_STATUS_IGNORE),
            );
            Error::from_mpi_res(res).map(|()| status.map(|s| Status::from_raw(s.assume_init())))
        }
    );
    #[inline]
    pub fn recv<B: BufferMut>(
        &self,
        buffer: B,
        tag: Tag,
        status_ignore: bool,
    ) -> RmpiResult<Option<Status>> {
        unsafe {
            self.recv_with(
                |buf, count, datatype, rank, tag, comm, status| {
                    MPI_Recv(buf, count, datatype, rank, tag, comm, status)
                },
                buffer,
                tag,
                status_ignore,
            )
        }
    }
}

use std::{mem::MaybeUninit, os::raw::*};

local_mod!(
    use mpi_sys::*;
    use crate::{BufferRef,BufferMut, Error, RmpiResult, Status, Tag};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn sendrecv_with<F, SB, RB>(
            mpi_sendrecv: F,

            send_buffer: SB,
            destination: Self,
            send_tag: Tag,

            mut recv_buffer: RB,
            source: Self,
            recv_tag: Tag,
            status_ignore: bool,
        ) -> RmpiResult<Option<Status>>
        where
            SB: BufferRef ,
            RB: BufferMut ,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                *mut c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
                *mut MPI_Status,
            ) -> c_int,
        {
            debug_assert_eq!(destination.communicator, source.communicator);
            let communicator = destination.communicator;
            let mut status = if status_ignore {
                None
            } else {
                Some(MaybeUninit::uninit())
            };
            let (send_buf, send_count) = send_buffer.as_raw();
            let (recv_buf, recv_count) = recv_buffer.as_raw_mut();
            let res = mpi_sendrecv(
                send_buf,
                send_count,
                send_buffer.item_datatype(),
                destination.rank,
                *send_tag,
                recv_buf,
                recv_count,
                recv_buffer.item_datatype(),
                source.rank,
                *recv_tag,
                communicator.as_raw(),
                status
                    .as_mut()
                    .map(|s| s.as_mut_ptr())
                    .unwrap_or(MPI_STATUS_IGNORE),
            );
            Error::from_mpi_res(res).map(|()| status.map(|s| Status::from_raw(s.assume_init())))
        }
    );
    #[inline]
    pub fn sendrecv<SB, RB>(
        send_buffer: SB,
        destination: Self,
        send_tag: Tag,

        recv_buffer: RB,
        source: Self,
        recv_tag: Tag,
        status_ignore: bool,
    ) -> RmpiResult<Option<Status>>
    where
        SB: BufferRef ,
        RB: BufferMut ,
    {
        unsafe {
            Self::sendrecv_with(
                |sendbuf,
                 sendcount,
                 sendtype,
                 dest,
                 sendtag,
                 recvbuf,
                 recvcount,
                 recvtype,
                 source,
                 recvtag,
                 comm,
                 status| {
                    MPI_Sendrecv(
                        sendbuf, sendcount, sendtype, dest, sendtag, recvbuf, recvcount, recvtype,
                        source, recvtag, comm, status,
                    )
                },
                send_buffer,
                destination,
                send_tag,
                recv_buffer,
                source,
                recv_tag,
                status_ignore,
            )
        }
    }
}

use std::{mem::MaybeUninit, os::raw::*};

local_mod!(
    use mpi_sys::*;
    use crate::{Buffer, Error, RmpiResult, Status, Tag};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn sendrecv_with<F, SB, RB>(
            mpi_sendrecv: F,

            send_buffer: &SB,
            destination: Self,
            send_tag: Tag,

            recv_buffer: &mut RB,
            source: Self,
            recv_tag: Tag,
        ) -> RmpiResult<Status>
        where
            SB: Buffer + ?Sized,
            RB: Buffer + ?Sized,
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
            let mut status = MaybeUninit::uninit();
            let (send_buf, send_count) = send_buffer.into_raw();
            let (recv_buf, recv_count) = recv_buffer.into_raw_mut();
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
                status.as_mut_ptr(),
            );
            Error::from_mpi_res(res).map(|()| Status::from_raw(status.assume_init()))
        }
    );
    #[inline]
    pub fn sendrecv<SB, RB>(
        send_buffer: &SB,
        destination: Self,
        send_tag: Tag,

        recv_buffer: &mut RB,
        source: Self,
        recv_tag: Tag,
    ) -> RmpiResult<Status>
    where
        SB: Buffer + ?Sized,
        RB: Buffer + ?Sized,
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
            )
        }
    }
}

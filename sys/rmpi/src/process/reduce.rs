use std::{os::raw::*, ptr};

local_mod!(
    use mpi_sys::*;
    use crate::{Buffer, Error, MpiOp, RmpiResult};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn reduce_with<F, B>(
            &self,
            mpi_reduce: F,
            send_buffer: &B,
            recv_buffer: Option<&mut B::Single>,
            op: MpiOp,
        ) -> RmpiResult
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *const c_void,
                *mut c_void,
                c_int,
                MPI_Datatype,
                MPI_Op,
                c_int,
                MPI_Comm,
            ) -> c_int,
        {
            let (sendbuf, sendcount) = send_buffer.into_raw();
            let recvbuf = recv_buffer
                .map(|rb| rb.into_raw_mut().0)
                .unwrap_or(ptr::null_mut());

            Error::from_mpi_res(mpi_reduce(
                sendbuf,
                recvbuf,
                sendcount,
                send_buffer.item_datatype(),
                op.into(),
                self.rank,
                self.communicator.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn reduce<B: Buffer + ?Sized>(
        &self,
        send_buffer: &B,
        recv_buffer: Option<&mut B::Single>,
        op: MpiOp,
    ) -> RmpiResult {
        unsafe {
            self.reduce_with(
                |sendbuf, recvbuf, count, dtype, op, root, comm| {
                    MPI_Reduce(sendbuf, recvbuf, count, dtype, op, root, comm)
                },
                send_buffer,
                recv_buffer,
                op,
            )
        }
    }
}

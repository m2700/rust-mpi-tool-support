use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{BufferRef, BufferMut, Error, MpiOp, RmpiResult};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn reduce_with<F, B>(
            &self,
            mpi_reduce: F,
            send_buffer: B,
            mut recv_buffer: B::Mut,
            op: MpiOp,
        ) -> RmpiResult
        where
            B: BufferRef,
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
            let (sendbuf, sendcount) = send_buffer.as_raw();
            let (recvbuf, recvcount) = recv_buffer.as_raw_mut();
            debug_assert!(self.rank != self.communicator.current_rank()? || recvcount == sendcount);

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
    pub fn reduce<B: BufferRef>(
        &self,
        send_buffer: B,
        recv_buffer: B::Mut,
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

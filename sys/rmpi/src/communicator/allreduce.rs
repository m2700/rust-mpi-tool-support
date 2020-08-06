use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{BufferRef, BufferMut, Error, RmpiResult, MpiOp};
);

use super::Communicator;

impl Communicator {
    tool_mode_item!(
        #[inline]
        pub unsafe fn allreduce_with<F, B>(
            &self,
            mpi_allreduce: F,
            send_buffer: B,
            mut recv_buffer: B::Mut,
            op: MpiOp,
        ) -> RmpiResult
        where
            B: BufferRef,
            F: FnOnce(*const c_void, *mut c_void, c_int, MPI_Datatype, MPI_Op, MPI_Comm) -> c_int,
        {
            let (sendbuf, sendcount) = send_buffer.as_raw();
            let (recvbuf, recvcount) = recv_buffer.as_raw_mut();
            debug_assert!(recvcount == sendcount);

            Error::from_mpi_res(mpi_allreduce(
                sendbuf,
                recvbuf,
                sendcount,
                send_buffer.item_datatype(),
                op.into(),
                self.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn allreduce<B: BufferRef>(
        &self,
        send_buffer: B,
        recv_buffer: B::Mut,
        op: MpiOp,
    ) -> RmpiResult {
        unsafe {
            self.allreduce_with(
                |sendbuf, recvbuf, count, dtype, op, comm| {
                    MPI_Allreduce(sendbuf, recvbuf, count, dtype, op, comm)
                },
                send_buffer,
                recv_buffer,
                op,
            )
        }
    }
}

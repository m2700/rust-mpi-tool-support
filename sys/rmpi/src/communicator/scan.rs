use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{BufferRef,  MpiOp, RmpiResult};
);

use super::Communicator;

impl<'ctx> Communicator<'ctx> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn scan_with<F, B>(
            &self,
            mpi_scan: F,
            send_buffer: B,
            recv_buffer: B::Mut,
            op: MpiOp,
        ) -> RmpiResult
        where
            B: BufferRef,
            F: FnOnce(*const c_void, *mut c_void, c_int, MPI_Datatype, MPI_Op, MPI_Comm) -> c_int,
        {
            self.allreduce_with(mpi_scan, send_buffer, recv_buffer, op)
        }
    );
    #[inline]
    pub fn scan<B: BufferRef>(&self, send_buffer: B, recv_buffer: B::Mut, op: MpiOp) -> RmpiResult {
        unsafe {
            self.scan_with(
                |sendbuf, recvbuf, count, dtype, op, comm| {
                    MPI_Scan(sendbuf, recvbuf, count, dtype, op, comm)
                },
                send_buffer,
                recv_buffer,
                op,
            )
        }
    }
}

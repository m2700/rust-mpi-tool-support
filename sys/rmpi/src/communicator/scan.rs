use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{Buffer, MpiOp, RmpiResult};
);

use super::Communicator;

impl Communicator {
    tool_mode_item!(
        #[inline]
        pub unsafe fn scan_with<F, B>(
            &self,
            mpi_scan: F,
            send_buffer: &B,
            recv_buffer: &mut B::Single,
            op: MpiOp,
        ) -> RmpiResult
        where
            B: Buffer + ?Sized,
            F: FnOnce(*const c_void, *mut c_void, c_int, MPI_Datatype, MPI_Op, MPI_Comm) -> c_int,
        {
            self.allreduce_with(mpi_scan, send_buffer, recv_buffer, op)
        }
    );
    #[inline]
    pub fn scan<B: Buffer + ?Sized>(
        &self,
        send_buffer: &B,
        recv_buffer: &mut B::Single,
        op: MpiOp,
    ) -> RmpiResult {
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

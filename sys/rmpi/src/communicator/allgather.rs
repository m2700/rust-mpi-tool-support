use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{BufferRef, BufferMut, Error, RmpiResult};
);

use super::Communicator;

impl<'ctx> Communicator<'ctx> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn allgather_with<F, SB, RB>(
            &self,
            mpi_allgather: F,
            send_buffer: SB,
            mut recv_buffer: RB,
        ) -> RmpiResult
        where
            SB: BufferRef,
            RB: BufferMut,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                *mut c_void,
                c_int,
                MPI_Datatype,
                MPI_Comm,
            ) -> c_int,
        {
            let (sendbuf, sendcount) = send_buffer.as_raw();
            let (recvbuf, recvcount) = recv_buffer.as_raw_mut();
            Error::from_mpi_res(mpi_allgather(
                sendbuf,
                sendcount,
                send_buffer.item_datatype(),
                recvbuf,
                recvcount,
                recv_buffer.item_datatype(),
                self.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn allgather<SB: BufferRef, RB: BufferMut>(
        &self,
        send_buffer: SB,
        recv_buffer: RB,
    ) -> RmpiResult {
        unsafe {
            self.allgather_with(
                |sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm| {
                    MPI_Allgather(
                        sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
                    )
                },
                send_buffer,
                recv_buffer,
            )
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn allgatherv_with<F, SB, RB>(
            &self,
            mpi_allgatherv: F,
            send_buffer: SB,
            recv_buffers: &mut [RB],
        ) -> RmpiResult
        where
            SB: BufferRef,
            RB: BufferMut,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                *mut c_void,
                *const c_int,
                *const c_int,
                MPI_Datatype,
                MPI_Comm,
            ) -> c_int,
        {
            debug_assert_eq!(recv_buffers.len() as c_int, self.size()?);

            let (sendbuf, sendcount) = send_buffer.as_raw();

            let recv_datatype_size = recv_buffers[0].datatype_size()? as usize;
            let recvbuf_ptr = recv_buffers
                .iter_mut()
                .map(|recv_buffer| recv_buffer.as_mut_ptr())
                .min()
                .unwrap();
            debug_assert!(recv_buffers
                .iter()
                .all(|buf| buf.item_datatype() == recv_buffers[0].item_datatype()));

            let [mut recv_displs, mut recv_counts] = [vec![], vec![]];
            for (recvbuf, recvcount) in recv_buffers
                .iter_mut()
                .map(|recv_buffer| (recv_buffer.as_mut_ptr(), recv_buffer.len()))
            {
                recv_displs.push(
                    (((recvbuf as usize) - (recvbuf_ptr as usize)) / recv_datatype_size) as c_int,
                );
                recv_counts.push(recvcount as c_int);
            }

            Error::from_mpi_res(mpi_allgatherv(
                sendbuf,
                sendcount,
                send_buffer.item_datatype(),
                recvbuf_ptr as *mut c_void,
                recv_counts.as_ptr(),
                recv_displs.as_ptr(),
                recv_buffers[0].item_datatype(),
                self.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn allgatherv<SB: BufferRef, RB: BufferMut>(
        &self,
        send_buffer: SB,
        recv_buffers: &mut [RB],
    ) -> RmpiResult {
        unsafe {
            self.allgatherv_with(
                |sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm| {
                    MPI_Allgatherv(
                        sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm,
                    )
                },
                send_buffer,
                recv_buffers,
            )
        }
    }
}

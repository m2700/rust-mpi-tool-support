use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{BufferRef, BufferMut, Error, RmpiResult};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn scatter_with<F, SB, RB>(
            &self,
            mpi_scatter: F,
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
                c_int,
                MPI_Comm,
            ) -> c_int,
        {
            let (sendbuf, sendcount) = send_buffer.as_raw();
            let (recvbuf, recvcount) = recv_buffer.as_raw_mut();
            Error::from_mpi_res(mpi_scatter(
                sendbuf,
                sendcount,
                send_buffer.item_datatype(),
                recvbuf,
                recvcount,
                recv_buffer.item_datatype(),
                self.rank,
                self.communicator.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn scatter<SB: BufferRef, RB: BufferMut>(
        &self,
        send_buffer: SB,
        recv_buffer: RB,
    ) -> RmpiResult {
        unsafe {
            self.scatter_with(
                |sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm| {
                    MPI_Scatter(
                        sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm,
                    )
                },
                send_buffer,
                recv_buffer,
            )
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn scatterv_with<F, SB, RB>(
            &self,
            mpi_scatterv: F,
            send_buffers: &[SB],
            mut recv_buffer: RB,
        ) -> RmpiResult
        where
            SB: BufferRef,
            RB: BufferMut,
            F: FnOnce(
                *const c_void,
                *const c_int,
                *const c_int,
                MPI_Datatype,
                *mut c_void,
                c_int,
                MPI_Datatype,
                c_int,
                MPI_Comm,
            ) -> c_int,
        {
            if self.communicator.current_rank()? == self.rank {
                debug_assert_eq!(send_buffers.len() as c_int, self.communicator.size()?);
            }

            let (recvbuf, recvcount) = recv_buffer.as_raw_mut();

            let send_datatype_size = send_buffers[0].datatype_size()? as usize;
            let sendbuf_ptr = send_buffers
                .iter()
                .map(|send_buffer| send_buffer.as_ptr())
                .min()
                .unwrap();
            let send_datatype = send_buffers
                .get(0)
                .map(|buf| buf.item_datatype())
                .unwrap_or_default();
            debug_assert!(send_buffers
                .iter()
                .all(|buf| buf.item_datatype() == send_datatype));

            let [mut send_displs, mut send_counts] = [vec![], vec![]];
            for (sendbuf, sendcount) in send_buffers
                .iter()
                .map(|send_buffer| (send_buffer.as_ptr(), send_buffer.len()))
            {
                send_displs.push(
                    (((sendbuf as usize) - (sendbuf_ptr as usize)) / send_datatype_size) as c_int,
                );
                send_counts.push(sendcount as c_int);
            }

            Error::from_mpi_res(mpi_scatterv(
                sendbuf_ptr as *mut c_void,
                send_counts.as_ptr(),
                send_displs.as_ptr(),
                send_datatype,
                recvbuf,
                recvcount,
                recv_buffer.item_datatype(),
                self.rank,
                self.communicator.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn scatterv<SB: BufferRef, RB: BufferMut>(
        &self,
        send_buffers: &[SB],
        recv_buffer: RB,
    ) -> RmpiResult {
        unsafe {
            self.scatterv_with(
                |sendbuf,
                 sendcounts,
                 displs,
                 sendtype,
                 recvbuf,
                 recvcount,
                 recvtype,
                 root,
                 comm| {
                    MPI_Scatterv(
                        sendbuf, sendcounts, displs, sendtype, recvbuf, recvcount, recvtype, root,
                        comm,
                    )
                },
                send_buffers,
                recv_buffer,
            )
        }
    }
}

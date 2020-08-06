use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{BufferRef, BufferMut, Error, RmpiResult};
);

use super::Communicator;

impl Communicator {
    tool_mode_item!(
        #[inline]
        pub unsafe fn alltoall_with<F, SB, RB>(
            &self,
            mpi_alltoall: F,
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
            Error::from_mpi_res(mpi_alltoall(
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
    pub fn alltoall<SB: BufferRef, RB: BufferMut>(
        &self,
        send_buffer: SB,
        recv_buffer: RB,
    ) -> RmpiResult {
        unsafe {
            self.alltoall_with(
                |sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm| {
                    MPI_Alltoall(
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
        pub unsafe fn alltoallv_with<F, SB, RB>(
            &self,
            mpi_alltoallv: F,
            send_buffers: &[SB],
            recv_buffers: &mut [RB],
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
                *const c_int,
                *const c_int,
                MPI_Datatype,
                MPI_Comm,
            ) -> c_int,
        {
            let send_datatype_size = send_buffers[0].datatype_size()? as usize;
            let sendbuf_ptr = send_buffers
                .iter()
                .map(|send_buffer| send_buffer.as_ptr())
                .min()
                .unwrap();
            let send_datatype = send_buffers[0].item_datatype();
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

            Error::from_mpi_res(mpi_alltoallv(
                sendbuf_ptr as *const c_void,
                send_counts.as_ptr(),
                send_displs.as_ptr(),
                send_datatype,
                recvbuf_ptr as *mut c_void,
                recv_counts.as_ptr(),
                recv_displs.as_ptr(),
                recv_buffers[0].item_datatype(),
                self.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn alltoallv<SB: BufferRef, RB: BufferMut>(
        &self,
        send_buffers: &[SB],
        recv_buffers: &mut [RB],
    ) -> RmpiResult {
        unsafe {
            self.alltoallv_with(
                |sendbuf,
                 sendcounts,
                 sdispls,
                 sendtype,
                 recvbuf,
                 recvcounts,
                 rdispls,
                 recvtype,
                 comm| {
                    MPI_Alltoallv(
                        sendbuf, sendcounts, sdispls, sendtype, recvbuf, recvcounts, rdispls,
                        recvtype, comm,
                    )
                },
                send_buffers,
                recv_buffers,
            )
        }
    }
}

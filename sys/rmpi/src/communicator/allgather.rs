use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{Buffer, Error, RmpiResult};
);

use super::Communicator;

impl Communicator {
    tool_mode_item!(
        #[inline]
        pub unsafe fn allgather_with<F, SB, RB>(
            &self,
            mpi_allgather: F,
            send_buffer: &SB,
            recv_buffer: &mut RB,
        ) -> RmpiResult
        where
            SB: Buffer + ?Sized,
            RB: Buffer + ?Sized,
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
            let (sendbuf, sendcount) = send_buffer.into_raw();
            let (recvbuf, recvcount) = recv_buffer.into_raw_mut();
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
    pub fn allgather<SB: Buffer + ?Sized, RB: Buffer + ?Sized>(
        &self,
        send_buffer: &SB,
        recv_buffer: &mut RB,
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
            send_buffer: &SB,
            recv_buffers: &mut [&mut RB],
        ) -> RmpiResult
        where
            SB: Buffer + ?Sized,
            RB: Buffer + ?Sized,
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
            debug_assert_eq!(Ok(recv_buffers.len() as c_int), self.size());

            let (sendbuf, sendcount) = send_buffer.into_raw();

            let recvbuf_ptr = recv_buffers
                .iter_mut()
                .map(|recv_buffer| recv_buffer.as_mut_ptr())
                .min()
                .unwrap();
            let recv_datatype = recv_buffers[0].item_datatype();
            debug_assert!(recv_buffers
                .iter()
                .all(|buf| buf.item_datatype() == recv_datatype));

            let [mut recv_displs, mut recv_counts] = [vec![], vec![]];
            for (recvbuf, recvcount) in recv_buffers
                .iter_mut()
                .map(|recv_buffer| (recv_buffer.as_mut_ptr(), recv_buffer.len()))
            {
                recv_displs.push(((recvbuf as usize) - (recvbuf_ptr as usize)) as c_int);
                recv_counts.push(recvcount as c_int);
            }

            Error::from_mpi_res(mpi_allgatherv(
                sendbuf,
                sendcount,
                send_buffer.item_datatype(),
                recvbuf_ptr as *mut c_void,
                recv_counts.as_ptr(),
                recv_displs.as_ptr(),
                recv_datatype,
                self.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn allgatherv<SB: Buffer + ?Sized, RB: Buffer + ?Sized>(
        &self,
        send_buffer: &SB,
        recv_buffers: &mut [&mut RB],
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

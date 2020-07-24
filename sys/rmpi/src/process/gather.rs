use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{Buffer, Error, RmpiResult};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn gather_with<F, SB, RB>(
            &self,
            mpi_gather: F,
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
                c_int,
                MPI_Comm,
            ) -> c_int,
        {
            let (sendbuf, sendcount) = send_buffer.into_raw();
            let (recvbuf, recvcount) = recv_buffer.into_raw_mut();
            Error::from_mpi_res(mpi_gather(
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
    pub fn gather<SB: Buffer + ?Sized, RB: Buffer + ?Sized>(
        &self,
        send_buffer: &SB,
        recv_buffer: &mut RB,
    ) -> RmpiResult {
        unsafe {
            self.gather_with(
                |sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm| {
                    MPI_Gather(
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
        pub unsafe fn gatherv_with<F, SB, RB>(
            &self,
            mpi_gatherv: F,
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
                c_int,
                MPI_Comm,
            ) -> c_int,
        {
            debug_assert_eq!(Ok(recv_buffers.len() as c_int), self.communicator.size());

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

            Error::from_mpi_res(mpi_gatherv(
                sendbuf,
                sendcount,
                send_buffer.item_datatype(),
                recvbuf_ptr as *mut c_void,
                recv_counts.as_ptr(),
                recv_displs.as_ptr(),
                recv_datatype,
                self.rank,
                self.communicator.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn gatherv<SB: Buffer + ?Sized, RB: Buffer + ?Sized>(
        &self,
        send_buffer: &SB,
        recv_buffers: &mut [&mut RB],
    ) -> RmpiResult {
        unsafe {
            self.gatherv_with(
                |sendbuf,
                 sendcount,
                 sendtype,
                 recvbuf,
                 recvcounts,
                 displs,
                 recvtype,
                 root,
                 comm| {
                    MPI_Gatherv(
                        sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, root,
                        comm,
                    )
                },
                send_buffer,
                recv_buffers,
            )
        }
    }
}

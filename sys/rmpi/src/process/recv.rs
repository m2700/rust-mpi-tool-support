use std::{mem::MaybeUninit, os::raw::*};

local_mod!(
    use mpi_sys::*;
    use crate::{Buffer, Error, RmpiResult, Status, Tag};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn recv_with<F, B>(
            &self,
            mpi_recv: F,
            buffer: &mut B,
            tag: Tag,
        ) -> RmpiResult<Status>
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *mut c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
                *mut MPI_Status,
            ) -> c_int,
        {
            let mut status = MaybeUninit::uninit();
            let (buf, count) = buffer.into_raw_mut();
            let res = mpi_recv(
                buf,
                count,
                buffer.item_datatype(),
                self.rank,
                *tag,
                self.communicator.as_raw(),
                status.as_mut_ptr(),
            );
            Error::from_mpi_res(res).map(|()| Status::from_raw(status.assume_init()))
        }
    );
    #[inline]
    pub fn recv<B: Buffer + ?Sized>(&self, buffer: &mut B, tag: Tag) -> RmpiResult<Status> {
        unsafe {
            self.recv_with(
                |buf, count, datatype, rank, tag, comm, status| {
                    MPI_Recv(buf, count, datatype, rank, tag, comm, status)
                },
                buffer,
                tag,
            )
        }
    }
}

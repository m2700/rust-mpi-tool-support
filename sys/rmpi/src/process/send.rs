use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{BufferRef, Error, RmpiResult, Tag};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn send_with<F, B>(&self, mpi_send: F, buffer: B, tag: Tag) -> RmpiResult
        where
            B: BufferRef,
            F: FnOnce(*const c_void, c_int, MPI_Datatype, c_int, c_int, MPI_Comm) -> c_int,
        {
            let (buf, count) = buffer.as_raw();
            Error::from_mpi_res(mpi_send(
                buf,
                count,
                buffer.item_datatype(),
                self.rank,
                *tag,
                self.communicator.as_raw(),
            ))
        }
    );
    #[inline]
    pub fn send<B: BufferRef>(&self, buffer: B, tag: Tag) -> RmpiResult {
        unsafe {
            self.send_with(
                |buf, count, datatype, rank, tag, comm| {
                    MPI_Send(buf, count, datatype, rank, tag, comm)
                },
                buffer,
                tag,
            )
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn bsend_with<F, B>(&self, mpi_bsend: F, buffer: B, tag: Tag) -> RmpiResult
        where
            B: BufferRef,
            F: FnOnce(*const c_void, c_int, MPI_Datatype, c_int, c_int, MPI_Comm) -> c_int,
        {
            self.send_with(mpi_bsend, buffer, tag)
        }
    );
    #[inline]
    pub fn bsend<B: BufferRef>(&self, buffer: B, tag: Tag) -> RmpiResult {
        unsafe {
            self.bsend_with(
                |buf, count, datatype, rank, tag, comm| {
                    MPI_Bsend(buf, count, datatype, rank, tag, comm)
                },
                buffer,
                tag,
            )
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn ssend_with<F, B>(&self, mpi_bsend: F, buffer: B, tag: Tag) -> RmpiResult
        where
            B: BufferRef,
            F: FnOnce(*const c_void, c_int, MPI_Datatype, c_int, c_int, MPI_Comm) -> c_int,
        {
            self.send_with(mpi_bsend, buffer, tag)
        }
    );
    #[inline]
    pub fn ssend<B: BufferRef>(&self, buffer: B, tag: Tag) -> RmpiResult {
        unsafe {
            self.ssend_with(
                |buf, count, datatype, rank, tag, comm| {
                    MPI_Ssend(buf, count, datatype, rank, tag, comm)
                },
                buffer,
                tag,
            )
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn rsend_with<F, B>(&self, mpi_bsend: F, buffer: B, tag: Tag) -> RmpiResult
        where
            B: BufferRef,
            F: FnOnce(*const c_void, c_int, MPI_Datatype, c_int, c_int, MPI_Comm) -> c_int,
        {
            self.send_with(mpi_bsend, buffer, tag)
        }
    );
    #[inline]
    pub fn rsend<B: BufferRef>(&self, buffer: B, tag: Tag) -> RmpiResult {
        unsafe {
            self.rsend_with(
                |buf, count, datatype, rank, tag, comm| {
                    MPI_Rsend(buf, count, datatype, rank, tag, comm)
                },
                buffer,
                tag,
            )
        }
    }
}

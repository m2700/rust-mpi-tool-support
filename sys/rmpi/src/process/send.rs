use std::os::raw::*;

use mpi_sys::*;

use crate::{Buffer, Error, RmpiResult, Tag};

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item! {
        #[inline]
        pub unsafe fn send_with<F, B>(&mut self, mpi_send: F, buffer: &B, tag: Tag) -> RmpiResult
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
            ) -> c_int,{
            let (buf, count) = buffer.into_raw();
            Error::from_mpi_res(
                mpi_send(
                    buf,
                    count,
                    buffer.item_datatype(),
                    self.rank,
                    *tag,
                    self.communicator.as_raw(),
                )
            )
        }
    }
    #[inline]
    pub fn send<B: Buffer + ?Sized>(&mut self, buffer: &B, tag: Tag) -> RmpiResult {
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

    tool_mode_item! {
        #[inline]
        pub unsafe fn bsend_with<F, B>(&mut self, mpi_bsend: F, buffer: &B, tag: Tag) -> RmpiResult
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
            ) -> c_int,{
            self.send_with(mpi_bsend, buffer, tag)
        }
    }
    #[inline]
    pub fn bsend<B: Buffer + ?Sized>(&mut self, buffer: &B, tag: Tag) -> RmpiResult {
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

    tool_mode_item! {
        #[inline]
        pub unsafe fn ssend_with<F, B>(&mut self, mpi_bsend: F, buffer: &B, tag: Tag) -> RmpiResult
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
            ) -> c_int,{
            self.send_with(mpi_bsend, buffer, tag)
        }
    }
    #[inline]
    pub fn ssend<B: Buffer + ?Sized>(&mut self, buffer: &B, tag: Tag) -> RmpiResult {
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

    tool_mode_item! {
        #[inline]
        pub unsafe fn rsend_with<F, B>(&mut self, mpi_bsend: F, buffer: &B, tag: Tag) -> RmpiResult
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
            ) -> c_int,{
            self.send_with(mpi_bsend, buffer, tag)
        }
    }
    #[inline]
    pub fn rsend<B: Buffer + ?Sized>(&mut self, buffer: &B, tag: Tag) -> RmpiResult {
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

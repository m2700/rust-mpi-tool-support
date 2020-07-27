use std::os::raw::*;

local_mod!(
    use mpi_sys::*;
    use crate::{request::Request, Buffer, Error, RmpiResult, Tag};
);

use super::Process;

impl<'c> Process<'c> {
    tool_mode_item!(
        #[inline]
        pub unsafe fn isend_with<'b, F, B>(
            &self,
            mpi_isend: F,
            buffer: &'b B,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
                *mut MPI_Request,
            ) -> c_int,
        {
            let (buf, count) = buffer.into_raw();
            let mut mpi_request = 0;
            Error::from_mpi_res(mpi_isend(
                buf,
                count,
                buffer.item_datatype(),
                self.rank,
                *tag,
                self.communicator.as_raw(),
                &mut mpi_request,
            ))
            .map(|()| Request::from_raw(mpi_request))
        }
    );
    #[inline]
    pub fn isend<'b, B: Buffer + ?Sized>(
        &self,
        buffer: &'b B,
        tag: Tag,
    ) -> RmpiResult<Request<'b>> {
        unsafe {
            self.isend_with(
                |buf, count, datatype, rank, tag, comm, request| {
                    MPI_Isend(buf, count, datatype, rank, tag, comm, request)
                },
                buffer,
                tag,
            )
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn ibsend_with<'b, F, B>(
            &self,
            mpi_ibsend: F,
            buffer: &'b B,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
                *mut MPI_Request,
            ) -> c_int,
        {
            let (buf, count) = buffer.into_raw();
            let mut mpi_request = 0;
            Error::from_mpi_res(mpi_ibsend(
                buf,
                count,
                buffer.item_datatype(),
                self.rank,
                *tag,
                self.communicator.as_raw(),
                &mut mpi_request,
            ))
            .map(|()| Request::from_raw(mpi_request))
        }
    );
    #[inline]
    pub fn ibsend<'b, B: Buffer + ?Sized>(
        &self,
        buffer: &'b B,
        tag: Tag,
    ) -> RmpiResult<Request<'b>> {
        unsafe {
            self.ibsend_with(
                |buf, count, datatype, rank, tag, comm, request| {
                    MPI_Ibsend(buf, count, datatype, rank, tag, comm, request)
                },
                buffer,
                tag,
            )
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn issend_with<'b, F, B>(
            &self,
            mpi_issend: F,
            buffer: &'b B,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
                *mut MPI_Request,
            ) -> c_int,
        {
            let (buf, count) = buffer.into_raw();
            let mut mpi_request = 0;
            Error::from_mpi_res(mpi_issend(
                buf,
                count,
                buffer.item_datatype(),
                self.rank,
                *tag,
                self.communicator.as_raw(),
                &mut mpi_request,
            ))
            .map(|()| Request::from_raw(mpi_request))
        }
    );
    #[inline]
    pub fn issend<'b, B: Buffer + ?Sized>(
        &self,
        buffer: &'b B,
        tag: Tag,
    ) -> RmpiResult<Request<'b>> {
        unsafe {
            self.issend_with(
                |buf, count, datatype, rank, tag, comm, request| {
                    MPI_Issend(buf, count, datatype, rank, tag, comm, request)
                },
                buffer,
                tag,
            )
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn irsend_with<'b, F, B>(
            &self,
            mpi_irsend: F,
            buffer: &'b B,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            B: Buffer + ?Sized,
            F: FnOnce(
                *const c_void,
                c_int,
                MPI_Datatype,
                c_int,
                c_int,
                MPI_Comm,
                *mut MPI_Request,
            ) -> c_int,
        {
            let (buf, count) = buffer.into_raw();
            let mut mpi_request = 0;
            Error::from_mpi_res(mpi_irsend(
                buf,
                count,
                buffer.item_datatype(),
                self.rank,
                *tag,
                self.communicator.as_raw(),
                &mut mpi_request,
            ))
            .map(|()| Request::from_raw(mpi_request))
        }
    );
    #[inline]
    pub fn irsend<'b, B: Buffer + ?Sized>(
        &self,
        buffer: &'b B,
        tag: Tag,
    ) -> RmpiResult<Request<'b>> {
        unsafe {
            self.irsend_with(
                |buf, count, datatype, rank, tag, comm, request| {
                    MPI_Irsend(buf, count, datatype, rank, tag, comm, request)
                },
                buffer,
                tag,
            )
        }
    }
}

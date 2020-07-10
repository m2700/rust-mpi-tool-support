use std::{os::raw::*, ptr, slice};

use mpi_sys::*;
use rmpi::{
    request::{Request, RequestSlice},
    Buffer, Communicator, MpiDatatype, Status,
};

use crate::{RawMpiInterceptionLayer, UnsafeBox};

use super::MpiInterceptionLayer;

impl<T> RawMpiInterceptionLayer for T
where
    T: MpiInterceptionLayer,
{
    trait_layer_function! {
        #[inline]
        fn send(
            unsafe next_f,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
        ) -> c_int
        {
            unsafe_with_buf!(
                (buf,count,datatype) => buffer => {
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::send(
                            |buf, mut dest, tag| {
                                unsafe{dest.send_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer, unsafe{Communicator::from_raw(comm)}.get_process(dest), tag.into()
                        )
                    )
                }
            )
        }
        #[inline]
        fn bsend(
            unsafe next_f,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
        ) -> c_int
        {
            unsafe_with_buf!(
                (buf,count,datatype) => buffer => {
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::bsend(
                            |buf, mut dest, tag| {
                                unsafe{dest.bsend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer, unsafe{Communicator::from_raw(comm)}.get_process(dest), tag.into()
                        )
                    )
                }
            )
        }
        #[inline]
        fn ssend(
            unsafe next_f,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
        ) -> c_int
        {
            unsafe_with_buf!(
                (buf,count,datatype) => buffer => {
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::ssend(
                            |buf, mut dest, tag| {
                                unsafe{dest.ssend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer, unsafe{Communicator::from_raw(comm)}.get_process(dest), tag.into()
                        )
                    )
                }
            )
        }
        #[inline]
        fn rsend(
            unsafe next_f,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
        ) -> c_int
        {
            unsafe_with_buf!(
                (buf,count,datatype) => buffer => {
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::rsend(
                            |buf, mut dest, tag| {
                                unsafe{dest.rsend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer, unsafe{Communicator::from_raw(comm)}.get_process(dest), tag.into()
                        )
                    )
                }
            )
        }

        #[inline]
        fn isend(
            unsafe next_f,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
            request: *mut MPI_Request
        ) -> c_int
        {
            unsafe_with_buf!(
                (buf,count,datatype) => buffer => {
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::isend(
                            |buf, mut dest, tag| {
                                unsafe{dest.isend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw(comm)}.get_process(dest),
                            tag.into()
                        ).map(|rq|{
                            unsafe{request.write(rq.into_raw())};
                            ()
                        })
                    )
                }
            )
        }
        #[inline]
        fn ibsend(
            unsafe next_f,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
            request: *mut MPI_Request
        ) -> c_int
        {
            unsafe_with_buf!(
                (buf,count,datatype) => buffer => {
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::ibsend(
                            |buf, mut dest, tag| {
                                unsafe{dest.ibsend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw(comm)}.get_process(dest),
                            tag.into()
                        ).map(|rq|{
                            unsafe{request.write(rq.into_raw())};
                            ()
                        })
                    )
                }
            )
        }
        #[inline]
        fn issend(
            unsafe next_f,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
            request: *mut MPI_Request
        ) -> c_int
        {
            unsafe_with_buf!(
                (buf,count,datatype) => buffer => {
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::issend(
                            |buf, mut dest, tag| {
                                unsafe{dest.issend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw(comm)}.get_process(dest),
                            tag.into()
                        ).map(|rq|{
                            unsafe{request.write(rq.into_raw())};
                            ()
                        })
                    )
                }
            )
        }
        #[inline]
        fn irsend(
            unsafe next_f,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
            request: *mut MPI_Request
        ) -> c_int
        {
            unsafe_with_buf!(
                (buf,count,datatype) => buffer => {
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::irsend(
                            |buf, mut dest, tag| {
                                unsafe{dest.irsend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw(comm)}.get_process(dest),
                            tag.into()
                        ).map(|rq|{
                            unsafe{request.write(rq.into_raw())};
                            ()
                        })
                    )
                }
            )
        }

        #[inline]
        fn recv(
            unsafe next_f,
            buf: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            source: c_int,
            tag: c_int,
            comm: MPI_Comm,
            status: *mut MPI_Status,
        ) -> c_int
        {
            unsafe_with_buf_mut!(
                (buf,count,datatype) => buffer => {
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::recv(
                            |buf, mut dest, tag| {
                                unsafe{dest.recv_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer, unsafe{Communicator::from_raw(comm)}.get_process(source), tag.into()
                        ).map(|status_ret|{
                            unsafe{status.write(status_ret.into_raw())};
                            ()
                        })
                    )
                }
            )
        }

        #[inline]
        fn get_count(
            unsafe next_f,
            status: *const MPI_Status,
            datatype: MPI_Datatype,
            count: *mut c_int,
        ) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                define_datatype! {
                    type DT = datatype;
                    <Self as MpiInterceptionLayer>::get_count::<_, DT>(
                        |status| unsafe{status.get_count_with::<_, DT>(next_f.unwrap())},
                        unsafe { &Status::from_raw(*status)}
                    )
                }
                .map(|cnt| {
                    unsafe { count.write(cnt) };
                    ()
                }),
            )
        }

        #[inline]
        fn buffer_attach(unsafe next_f, buffer: *mut c_void, size: c_int) -> c_int
        {
            let buffer = buffer as *mut *mut u8;
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::buffer_attach(
                |buffer| {
                    let (ptr, len) = buffer.into_raw_mut();
                    rmpi::Error::from_mpi_res(unsafe { next_f.unwrap()(ptr, len) })
                },
                unsafe { slice::from_raw_parts_mut(*buffer, size as usize) },
            ))
        }
        #[inline]
        fn buffer_detach(unsafe next_f, buffer: *mut c_void, size: *mut c_int) -> c_int
        {
            let buffer = buffer as *mut *mut u8;
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::buffer_detach(
                || {
                    let buffer = &mut ptr::null_mut();
                    let size = &mut 0;
                    rmpi::Error::from_mpi_res(
                        unsafe { next_f.unwrap()(buffer as *mut *mut _ as *mut _, size) }).map(|()|{
                            let buffer = buffer as *mut *mut u8;
                            unsafe { slice::from_raw_parts_mut(*buffer, *size as usize) }
                        }
                    )
                },
            ).map(|buf|{
                unsafe{
                    buffer.write(buf.as_mut_ptr());
                    size.write(buf.len() as c_int);
                }
            }))
        }

        #[inline]
        fn wait(unsafe next_f, request: *mut MPI_Request, status: *mut MPI_Status) -> c_int
        {
            let mut rmpi_req = unsafe{Request::from_raw(*request)};
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::wait(
                    |request| {
                        unsafe{
                            request.wait_with(
                                next_f.unwrap(),
                            )
                        }
                    },
                    &mut rmpi_req
                ).map(|out_status|{
                    unsafe{
                        request.write(rmpi_req.into_raw());
                        status.write(out_status.into_raw());
                    }
                })
            )
        }
        #[inline]
        fn waitany(
            unsafe next_f,
            count: c_int,
            array_of_requests: *mut MPI_Request,
            indx: *mut c_int,
            status: *mut MPI_Status,
        ) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::waitany(
                    |req_slc| {
                        unsafe {
                            req_slc.waitany_with(next_f.unwrap())
                        }
                    },
                    unsafe{
                        RequestSlice::from_raw_mut(slice::from_raw_parts_mut(array_of_requests, count as usize))
                    }
                ).map(|(out_indx, out_status)|{
                    unsafe {
                        indx.write(out_indx as c_int);
                        status.write(out_status.into_raw());
                    }
                })
            )
        }

        #[inline]
        fn test(
            unsafe next_f,
            request: *mut MPI_Request,
            flag: *mut c_int,
            status: *mut MPI_Status,
        ) -> c_int
        {
            let mut rmpi_req = unsafe{Request::from_raw(*request)};
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::test(
                    |request| {
                        unsafe{
                            request.test_with(
                                next_f.unwrap(),
                            )
                        }
                    },
                    &mut rmpi_req,
                ).map(|opt_out_status|{
                    match opt_out_status{
                        Some(out_status) => {
                            unsafe{
                                request.write(rmpi_req.into_raw());
                                flag.write(1);
                                status.write(out_status.into_raw());
                            }
                        }
                        None => {
                            unsafe{
                                request.write(rmpi_req.into_raw());
                                flag.write(0);
                            }
                        }
                    }
                })
            )
        }

        #[inline]
        fn request_free(unsafe next_f, request: *mut MPI_Request) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::free(
                    |request| {
                        unsafe{
                            request.free_with(
                                next_f.unwrap(),
                            )
                        }
                    },
                    unsafe{Request::from_raw(*request)}
                )
            )
        }

        #[inline]
        fn finalize(unsafe next_f) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::finalize(
                    || rmpi::Error::from_mpi_res(unsafe{next_f.unwrap()}())
                )
            )
        }
    }
}

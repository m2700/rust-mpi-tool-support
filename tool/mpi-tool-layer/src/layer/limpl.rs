use std::{
    mem::{forget, transmute},
    os::raw::*,
    ptr, slice,
};

use rmpi::pmpi_mode as rmpi;

use self::rmpi::{
    datatype::RawDatatype,
    request::{Request, RequestSlice},
    BufferMut, BufferRef, CStrMutPtr, Communicator, Group, Process, RmpiContext, Status,
    TypeDynamicBufferMut, TypeDynamicBufferRef,
};
use mpi_sys::pmpi::*;

use crate::{RawMpiInterceptionLayer, UnsafeBox};

use super::{buffers_from_displs, buffers_mut_from_displs, MpiInterceptionLayer};

impl<T> RawMpiInterceptionLayer for T
where
    T: MpiInterceptionLayer,
{
    trait_layer_function!(
        #[inline]
        fn init(next_f: UnsafeBox, argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int {
            let args: &mut [*mut c_char] = unsafe {
                match (argc.as_mut(), argv.as_mut()) {
                    (Some(&mut argc), Some(&mut argv)) if !argv.is_null() => {
                        slice::from_raw_parts_mut(argv, argc as usize)
                    }
                    _ => &mut [],
                }
            };
            let mut args: &mut [CStrMutPtr] = unsafe { transmute(args) };
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::init(
                    |args| unsafe { rmpi::init_with(next_f.unwrap(), args) },
                    &mut args,
                )
                .map(|context| {
                    forget(context);
                    if let (Some(argc), Some(argv)) = unsafe { (argc.as_mut(), argv.as_mut()) } {
                        *argc = args.len() as c_int;
                        *argv = args.as_mut_ptr() as *mut *mut c_char;
                    }
                }),
            )
        }
        #[inline]
        fn initialized(next_f: UnsafeBox, flag: *mut c_int) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::initialized(|| unsafe {
                    rmpi::initialized_with(next_f.unwrap())
                })
                .map(|is_init| unsafe { *flag = is_init as c_int }),
            )
        }
        #[inline]
        fn finalize(next_f: UnsafeBox) -> c_int {
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::finalize(
                |rmpi_ctx| unsafe { rmpi_ctx.finalize_with(next_f.unwrap()) },
                unsafe { RmpiContext::create_unchecked() },
            ))
        }
        #[inline]
        fn finalized(next_f: UnsafeBox, flag: *mut c_int) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::finalized(|| unsafe {
                    rmpi::finalized_with(next_f.unwrap())
                })
                .map(|is_finit| unsafe { *flag = is_finit as c_int }),
            )
        }

        #[inline]
        fn wtime(next_f: UnsafeBox) -> c_double {
            <Self as MpiInterceptionLayer>::wtime(
                |_rmpi_ctx| unsafe { (next_f.unwrap())() },
                unsafe { RmpiContext::create_unchecked_ref() },
            )
        }
        #[inline]
        fn wtick(next_f: UnsafeBox) -> c_double {
            <Self as MpiInterceptionLayer>::wtick(
                |_rmpi_ctx| unsafe { (next_f.unwrap())() },
                unsafe { RmpiContext::create_unchecked_ref() },
            )
        }

        #[inline]
        fn barrier(next_f: UnsafeBox, comm: MPI_Comm) -> c_int {
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::barrier(
                |_rmpi_ctx, comm| unsafe { comm.barrier_with(next_f.unwrap()) },
                unsafe { RmpiContext::create_unchecked_ref() },
                unsafe { Communicator::from_raw_ref(&comm) },
            ))
        }

        #[inline]
        fn group_incl(
            next_f: UnsafeBox,
            group: MPI_Group,
            n: c_int,
            ranks: *const c_int,
            newgroup: *mut MPI_Group,
        ) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::group_incl(
                    |_rmpi_ctx, group, ranks| unsafe { group.incl_with(next_f.unwrap(), ranks) },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe { Group::from_raw_ref(&group) },
                    unsafe { slice::from_raw_parts(ranks, n as usize) },
                )
                .map(|newgroup_output| {
                    unsafe { newgroup.write(newgroup_output.into_raw()) };
                }),
            )
        }
        #[inline]
        fn group_free(next_f: UnsafeBox, group: *mut MPI_Group) -> c_int {
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::group_free(
                |_rmpi_ctx, group| unsafe { group.free_with(next_f.unwrap()) },
                unsafe { RmpiContext::create_unchecked_ref() },
                unsafe { Group::from_raw(*group) },
            ))
        }

        #[inline]
        fn comm_size(next_f: UnsafeBox, comm: MPI_Comm, size: *mut c_int) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::comm_size(
                    |_rmpi_ctx, comm| unsafe { comm.size_with(next_f.unwrap()) },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe { Communicator::from_raw_ref(&comm) },
                )
                .map(|size_out| {
                    unsafe { size.write(size_out) };
                }),
            )
        }
        #[inline]
        fn comm_rank(next_f: UnsafeBox, comm: MPI_Comm, rank: *mut c_int) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::comm_rank(
                    |_rmpi_ctx, comm| unsafe { comm.current_rank_with(next_f.unwrap()) },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe { Communicator::from_raw_ref(&comm) },
                )
                .map(|rank_out| {
                    unsafe { rank.write(rank_out) };
                }),
            )
        }
        #[inline]
        fn comm_create(
            next_f: UnsafeBox,
            comm: MPI_Comm,
            group: MPI_Group,
            newcomm: *mut MPI_Comm,
        ) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::comm_create(
                    |_rmpi_ctx, comm, group| unsafe {
                        comm.create_subset_with(next_f.unwrap(), group)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe { Communicator::from_raw_ref(&comm) },
                    unsafe { Group::from_raw_ref(&group) },
                )
                .map(|newcomm_output| {
                    unsafe { newcomm.write(newcomm_output.into_raw()) };
                }),
            )
        }
        #[inline]
        fn comm_free(next_f: UnsafeBox, comm: *mut MPI_Comm) -> c_int {
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::comm_free(
                |_rmpi_ctx, comm| unsafe { comm.free_with(next_f.unwrap()) },
                unsafe { RmpiContext::create_unchecked_ref() },
                unsafe { Communicator::from_raw(*comm) },
            ))
        }

        #[inline]
        fn send(
            next_f: UnsafeBox,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferRef::from_raw_dynamic(buf, count, datatype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::send(
                |_rmpi_ctx, buf, dest, tag| unsafe { dest.send_with(next_f.unwrap(), buf, tag) },
                unsafe { RmpiContext::create_unchecked_ref() },
                buffer,
                unsafe { Communicator::from_raw_ref(&comm) }.get_process(dest),
                tag.into(),
            ))
        }
        #[inline]
        fn bsend(
            next_f: UnsafeBox,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferRef::from_raw_dynamic(buf, count, datatype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::bsend(
                |_rmpi_ctx, buf, dest, tag| unsafe { dest.bsend_with(next_f.unwrap(), buf, tag) },
                unsafe { RmpiContext::create_unchecked_ref() },
                buffer,
                unsafe { Communicator::from_raw_ref(&comm) }.get_process(dest),
                tag.into(),
            ))
        }
        #[inline]
        fn ssend(
            next_f: UnsafeBox,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferRef::from_raw_dynamic(buf, count, datatype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::ssend(
                |_rmpi_ctx, buf, dest, tag| unsafe { dest.ssend_with(next_f.unwrap(), buf, tag) },
                unsafe { RmpiContext::create_unchecked_ref() },
                buffer,
                unsafe { Communicator::from_raw_ref(&comm) }.get_process(dest),
                tag.into(),
            ))
        }
        #[inline]
        fn rsend(
            next_f: UnsafeBox,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferRef::from_raw_dynamic(buf, count, datatype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::rsend(
                |_rmpi_ctx, buf, dest, tag| unsafe { dest.rsend_with(next_f.unwrap(), buf, tag) },
                unsafe { RmpiContext::create_unchecked_ref() },
                buffer,
                unsafe { Communicator::from_raw_ref(&comm) }.get_process(dest),
                tag.into(),
            ))
        }

        #[inline]
        fn isend(
            next_f: UnsafeBox,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
            request: *mut MPI_Request,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferRef::from_raw_dynamic(buf, count, datatype) };
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::isend(
                    |_rmpi_ctx, buf, dest, tag| unsafe {
                        dest.isend_with(next_f.unwrap(), buf, tag)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    buffer,
                    unsafe { Communicator::from_raw_ref(&comm) }.get_process(dest),
                    tag.into(),
                )
                .map(|rq| {
                    unsafe { request.write(rq.into_raw()) };
                    ()
                }),
            )
        }
        #[inline]
        fn ibsend(
            next_f: UnsafeBox,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
            request: *mut MPI_Request,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferRef::from_raw_dynamic(buf, count, datatype) };
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::ibsend(
                    |_rmpi_ctx, buf, dest, tag| unsafe {
                        dest.ibsend_with(next_f.unwrap(), buf, tag)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    buffer,
                    unsafe { Communicator::from_raw_ref(&comm) }.get_process(dest),
                    tag.into(),
                )
                .map(|rq| {
                    unsafe { request.write(rq.into_raw()) };
                    ()
                }),
            )
        }
        #[inline]
        fn issend(
            next_f: UnsafeBox,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
            request: *mut MPI_Request,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferRef::from_raw_dynamic(buf, count, datatype) };
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::issend(
                    |_rmpi_ctx, buf, dest, tag| unsafe {
                        dest.issend_with(next_f.unwrap(), buf, tag)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    buffer,
                    unsafe { Communicator::from_raw_ref(&comm) }.get_process(dest),
                    tag.into(),
                )
                .map(|rq| {
                    unsafe { request.write(rq.into_raw()) };
                    ()
                }),
            )
        }
        #[inline]
        fn irsend(
            next_f: UnsafeBox,
            buf: *const c_void,
            count: c_int,
            datatype: MPI_Datatype,
            dest: c_int,
            tag: c_int,
            comm: MPI_Comm,
            request: *mut MPI_Request,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferRef::from_raw_dynamic(buf, count, datatype) };
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::irsend(
                    |_rmpi_ctx, buf, dest, tag| unsafe {
                        dest.irsend_with(next_f.unwrap(), buf, tag)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    buffer,
                    unsafe { Communicator::from_raw_ref(&comm) }.get_process(dest),
                    tag.into(),
                )
                .map(|rq| {
                    unsafe { request.write(rq.into_raw()) };
                    ()
                }),
            )
        }

        #[inline]
        fn recv(
            next_f: UnsafeBox,
            buf: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            source: c_int,
            tag: c_int,
            comm: MPI_Comm,
            status: *mut MPI_Status,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferMut::from_raw_dynamic(buf, count, datatype) };
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::recv(
                    |_rmpi_ctx, buf, dest, tag, status_ignore| unsafe {
                        dest.recv_with(next_f.unwrap(), buf, tag, status_ignore)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    buffer,
                    unsafe { Communicator::from_raw_ref(&comm) }.get_process(source),
                    tag.into(),
                    (status as *const MPI_Status) == MPI_STATUS_IGNORE,
                )
                .map(|opt_status_ret| {
                    if let Some(status_ret) = opt_status_ret {
                        unsafe { status.write(status_ret.into_raw()) };
                    }
                }),
            )
        }

        #[inline]
        fn irecv(
            next_f: UnsafeBox,
            buf: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            source: c_int,
            tag: c_int,
            comm: MPI_Comm,
            request: *mut MPI_Request,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferMut::from_raw_dynamic(buf, count, datatype) };
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::irecv(
                    |_rmpi_ctx, buf, source, tag| unsafe {
                        source.irecv_with(next_f.unwrap(), buf, tag)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    buffer,
                    unsafe { Communicator::from_raw_ref(&comm) }.get_process(source),
                    tag.into(),
                )
                .map(|rq| {
                    unsafe { request.write(rq.into_raw()) };
                    ()
                }),
            )
        }

        #[inline]
        fn sendrecv(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            dest: c_int,
            sendtag: c_int,
            recvbuf: *mut c_void,
            recvcount: c_int,
            recvtype: MPI_Datatype,
            source: c_int,
            recvtag: c_int,
            comm: MPI_Comm,
            status: *mut MPI_Status,
        ) -> c_int {
            let send_buffer =
                unsafe { TypeDynamicBufferRef::from_raw_dynamic(sendbuf, sendcount, sendtype) };
            let recv_buffer =
                unsafe { TypeDynamicBufferMut::from_raw_dynamic(recvbuf, recvcount, recvtype) };
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::sendrecv(
                    |_rmpi_ctx, sendbuf, dest, sendtag, recvbuf, src, recvtag, status_ignore| unsafe {
                        Process::sendrecv_with(
                            next_f.unwrap(),
                            sendbuf,
                            dest,
                            sendtag,
                            recvbuf,
                            src,
                            recvtag,
                            status_ignore,
                        )
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    send_buffer,
                    unsafe { Communicator::from_raw_ref(&comm) }.get_process(dest),
                    sendtag.into(),
                    recv_buffer,
                    unsafe { Communicator::from_raw_ref(&comm) }.get_process(source),
                    recvtag.into(),
                    status == MPI_STATUS_IGNORE,
                )
                .map(|opt_status_ret| {
                    if let Some(status_ret) = opt_status_ret {
                        unsafe { status.write(status_ret.into_raw()) };
                    }
                }),
            )
        }

        #[inline]
        fn get_count(
            next_f: UnsafeBox,
            status: *const MPI_Status,
            datatype: MPI_Datatype,
            count: *mut c_int,
        ) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::get_count(
                    |_rmpi_ctx, status, datatype| unsafe {
                        status.get_count_with(next_f.unwrap(), datatype)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe { &Status::from_raw(*status) },
                    &unsafe { RawDatatype::from_raw(datatype) },
                )
                .map(|cnt| match cnt {
                    Some(cnt) => unsafe { count.write(cnt) },
                    None => unsafe { count.write(MPI_UNDEFINED) },
                }),
            )
        }

        #[inline]
        fn buffer_attach(next_f: UnsafeBox, buffer: *mut c_void, size: c_int) -> c_int {
            let buffer = buffer as *mut *mut u8;
            rmpi::Error::result_into_mpi_res(unsafe {
                <Self as MpiInterceptionLayer>::buffer_attach(
                    |_rmpi_ctx, mut buffer| {
                        let (ptr, len) = buffer.as_raw_mut();
                        rmpi::Error::from_mpi_res(next_f.unwrap()(ptr, len))
                    },
                    RmpiContext::create_unchecked_ref(),
                    slice::from_raw_parts_mut(*buffer, size as usize),
                )
            })
        }
        #[inline]
        fn buffer_detach(next_f: UnsafeBox, buffer: *mut c_void, size: *mut c_int) -> c_int {
            let buffer = buffer as *mut *mut u8;
            rmpi::Error::result_into_mpi_res(unsafe {
                <Self as MpiInterceptionLayer>::buffer_detach(
                    |_rmpi_ctx| {
                        let buffer = &mut ptr::null_mut();
                        let size = &mut 0;
                        rmpi::Error::from_mpi_res(next_f.unwrap()(
                            buffer as *mut *mut _ as *mut _,
                            size,
                        ))
                        .map(|()| {
                            let buffer = buffer as *mut *mut u8;
                            slice::from_raw_parts_mut(*buffer, *size as usize)
                        })
                    },
                    RmpiContext::create_unchecked_ref(),
                )
                .map(|buf| {
                    buffer.write(buf.as_mut_ptr());
                    size.write(buf.len() as c_int);
                })
            })
        }

        #[inline]
        fn wait(next_f: UnsafeBox, request: *mut MPI_Request, status: *mut MPI_Status) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::wait(
                    |_rmpi_ctx, request, status_ignore| unsafe {
                        request.wait_with(next_f.unwrap(), status_ignore)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe { Request::from_raw(*request) },
                    status == MPI_STATUS_IGNORE,
                )
                .map(|(opt_out_status, opt_request_out)| unsafe {
                    request.write(
                        opt_request_out
                            .map(|out_request| out_request.into_raw())
                            .unwrap_or(MPI_REQUEST_NULL),
                    );
                    if let Some(out_status) = opt_out_status {
                        status.write(out_status.into_raw());
                    }
                }),
            )
        }
        #[inline]
        fn waitany(
            next_f: UnsafeBox,
            count: c_int,
            array_of_requests: *mut MPI_Request,
            indx: *mut c_int,
            status: *mut MPI_Status,
        ) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::waitany(
                    |_rmpi_ctx, req_slc, status_ignore| unsafe {
                        req_slc.waitany_with(next_f.unwrap(), status_ignore)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe {
                        RequestSlice::from_raw_mut(slice::from_raw_parts_mut(
                            array_of_requests,
                            count as usize,
                        ))
                    },
                    status == MPI_STATUS_IGNORE,
                )
                .map(|(out_indx, opt_out_status)| unsafe {
                    indx.write(out_indx as c_int);
                    if let Some(out_status) = opt_out_status {
                        status.write(out_status.into_raw());
                    }
                }),
            )
        }
        #[inline]
        fn waitall(
            next_f: UnsafeBox,
            count: c_int,
            array_of_requests: *mut MPI_Request,
            array_of_statuses: *mut MPI_Status,
        ) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::waitall(
                    |_rmpi_ctx, req_slc, status_slc| unsafe {
                        req_slc.waitall_with(next_f.unwrap(), status_slc)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe {
                        RequestSlice::from_raw_mut(slice::from_raw_parts_mut(
                            array_of_requests,
                            count as usize,
                        ))
                    },
                    if array_of_statuses as *mut MPI_Status == MPI_STATUSES_IGNORE {
                        &mut []
                    } else {
                        unsafe {
                            Status::from_raw_slice_mut_maybe_uninit(
                                array_of_statuses,
                                count as usize,
                            )
                        }
                    },
                )
                .map(|opt_out_statuses| {
                    if let Some(out_statuses) = opt_out_statuses {
                        debug_assert_eq!(
                            out_statuses.as_mut_ptr() as *mut MPI_Status,
                            array_of_statuses
                        );
                    }
                }),
            )
        }

        #[inline]
        fn test(
            next_f: UnsafeBox,
            request: *mut MPI_Request,
            flag: *mut c_int,
            status: *mut MPI_Status,
        ) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::test(
                    |_rmpi_ctx, request, status_ignore| unsafe {
                        request.test_with(next_f.unwrap(), status_ignore)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe { Request::from_raw(*request) },
                    status == MPI_STATUS_IGNORE,
                )
                .map(|test_result| match test_result {
                    Ok((opt_out_status, opt_request_out)) => unsafe {
                        request.write(
                            opt_request_out
                                .map(|out_request| out_request.into_raw())
                                .unwrap_or(MPI_REQUEST_NULL),
                        );
                        flag.write(1);
                        if let Some(out_status) = opt_out_status {
                            status.write(out_status.into_raw());
                        }
                    },
                    Err(out_request) => unsafe {
                        request.write(out_request.into_raw());
                        flag.write(0);
                    },
                }),
            )
        }
        #[inline]
        fn testany(
            next_f: UnsafeBox,
            count: c_int,
            array_of_requests: *mut MPI_Request,
            indx: *mut c_int,
            flag: *mut c_int,
            status: *mut MPI_Status,
        ) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::testany(
                    |_rmpi_ctx, req_slc, status_ignore| unsafe {
                        req_slc.testany_with(next_f.unwrap(), status_ignore)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe {
                        RequestSlice::from_raw_mut(slice::from_raw_parts_mut(
                            array_of_requests,
                            count as usize,
                        ))
                    },
                    status == MPI_STATUS_IGNORE,
                )
                .map(|opt_out| match opt_out {
                    Some((out_indx, opt_out_status)) => unsafe {
                        *flag = 1;
                        indx.write(out_indx as c_int);
                        if let Some(out_status) = opt_out_status {
                            status.write(out_status.into_raw());
                        }
                    },
                    None => unsafe { *flag = 0 },
                }),
            )
        }
        #[inline]
        fn testall(
            next_f: UnsafeBox,
            count: c_int,
            array_of_requests: *mut MPI_Request,
            flag: *mut c_int,
            array_of_statuses: *mut MPI_Status,
        ) -> c_int {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::testall(
                    |_rmpi_ctx, req_slc, status_slc| unsafe {
                        req_slc.testall_with(next_f.unwrap(), status_slc)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    unsafe {
                        RequestSlice::from_raw_mut(slice::from_raw_parts_mut(
                            array_of_requests,
                            count as usize,
                        ))
                    },
                    if array_of_statuses as *mut MPI_Status == MPI_STATUSES_IGNORE {
                        &mut []
                    } else {
                        unsafe {
                            Status::from_raw_slice_mut_maybe_uninit(
                                array_of_statuses,
                                count as usize,
                            )
                        }
                    },
                )
                .map(|opt_out_statuses| {
                    unsafe { *flag = opt_out_statuses.is_some() as c_int };
                    if let Some(out_statuses) = opt_out_statuses {
                        debug_assert_eq!(
                            out_statuses.as_mut_ptr() as *mut MPI_Status,
                            array_of_statuses
                        );
                    }
                }),
            )
        }

        #[inline]
        fn request_free(next_f: UnsafeBox, request: *mut MPI_Request) -> c_int {
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::request_free(
                |_rmpi_ctx, request| unsafe { request.free_with(next_f.unwrap()) },
                unsafe { RmpiContext::create_unchecked_ref() },
                unsafe { Request::from_raw(*request) },
            ))
        }
        #[inline]
        fn cancel(next_f: UnsafeBox, request: *mut MPI_Request) -> c_int {
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::cancel(
                |_rmpi_ctx, request| unsafe { request.cancel_with(next_f.unwrap()) },
                unsafe { RmpiContext::create_unchecked_ref() },
                unsafe { Request::from_raw(*request) },
            ))
        }

        #[inline]
        fn bcast(
            next_f: UnsafeBox,
            buffer: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            root: c_int,
            comm: MPI_Comm,
        ) -> c_int {
            let buffer = unsafe { TypeDynamicBufferMut::from_raw_dynamic(buffer, count, datatype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::bcast(
                |_rmpi_ctx, buf, root| unsafe { root.bcast_with(next_f.unwrap(), buf) },
                unsafe { RmpiContext::create_unchecked_ref() },
                buffer,
                unsafe { Communicator::from_raw_ref(&comm) }.get_process(root),
            ))
        }

        #[inline]
        fn gather(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcount: c_int,
            recvtype: MPI_Datatype,
            root: c_int,
            comm: MPI_Comm,
        ) -> c_int {
            let send_buffer =
                unsafe { TypeDynamicBufferRef::from_raw_dynamic(sendbuf, sendcount, sendtype) };
            let recv_buffer =
                unsafe { TypeDynamicBufferMut::from_raw_dynamic(recvbuf, recvcount, recvtype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::gather(
                |_rmpi_ctx, send_buffer, recv_buffer, root| unsafe {
                    root.gather_with(next_f.unwrap(), send_buffer, recv_buffer)
                },
                unsafe { RmpiContext::create_unchecked_ref() },
                send_buffer,
                recv_buffer,
                unsafe { Communicator::from_raw_ref(&comm) }.get_process(root),
            ))
        }
        #[inline]
        fn gatherv(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcounts: *const c_int,
            displs: *const c_int,
            recvtype: MPI_Datatype,
            root: c_int,
            comm: MPI_Comm,
        ) -> c_int {
            let send_buffer =
                unsafe { TypeDynamicBufferRef::from_raw_dynamic(sendbuf, sendcount, sendtype) };
            rmpi::Error::result_into_mpi_res((|| {
                let mut recv_buffers =
                    buffers_mut_from_displs(recvbuf, recvcounts, displs, recvtype, comm)?;
                <Self as MpiInterceptionLayer>::gatherv(
                    |_rmpi_ctx, send_buffer, recv_buffer, root| unsafe {
                        root.gatherv_with(next_f.unwrap(), send_buffer, recv_buffer)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    send_buffer,
                    &mut recv_buffers,
                    unsafe { Communicator::from_raw_ref(&comm) }.get_process(root),
                )
            })())
        }

        #[inline]
        fn allgather(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcount: c_int,
            recvtype: MPI_Datatype,
            comm: MPI_Comm,
        ) -> c_int {
            let send_buffer =
                unsafe { TypeDynamicBufferRef::from_raw_dynamic(sendbuf, sendcount, sendtype) };
            let recv_buffer =
                unsafe { TypeDynamicBufferMut::from_raw_dynamic(recvbuf, recvcount, recvtype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::allgather(
                |_rmpi_ctx, send_buffer, recv_buffer, communicator| unsafe {
                    communicator.allgather_with(next_f.unwrap(), send_buffer, recv_buffer)
                },
                unsafe { RmpiContext::create_unchecked_ref() },
                send_buffer,
                recv_buffer,
                unsafe { Communicator::from_raw_ref(&comm) },
            ))
        }
        #[inline]
        fn allgatherv(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcounts: *const c_int,
            displs: *const c_int,
            recvtype: MPI_Datatype,
            comm: MPI_Comm,
        ) -> c_int {
            let send_buffer =
                unsafe { TypeDynamicBufferRef::from_raw_dynamic(sendbuf, sendcount, sendtype) };

            rmpi::Error::result_into_mpi_res((|| {
                let mut recv_buffers =
                    buffers_mut_from_displs(recvbuf, recvcounts, displs, recvtype, comm)?;
                <Self as MpiInterceptionLayer>::allgatherv(
                    |_rmpi_ctx, send_buffer, recv_buffer, root| unsafe {
                        root.allgatherv_with(next_f.unwrap(), send_buffer, recv_buffer)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    send_buffer,
                    &mut recv_buffers,
                    unsafe { Communicator::from_raw_ref(&comm) },
                )
            })())
        }

        #[inline]
        fn alltoall(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcount: c_int,
            recvtype: MPI_Datatype,
            comm: MPI_Comm,
        ) -> c_int {
            let send_buffer =
                unsafe { TypeDynamicBufferRef::from_raw_dynamic(sendbuf, sendcount, sendtype) };
            let recv_buffer =
                unsafe { TypeDynamicBufferMut::from_raw_dynamic(recvbuf, recvcount, recvtype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::alltoall(
                |_rmpi_ctx, send_buffer, recv_buffer, communicator| unsafe {
                    communicator.alltoall_with(next_f.unwrap(), send_buffer, recv_buffer)
                },
                unsafe { RmpiContext::create_unchecked_ref() },
                send_buffer,
                recv_buffer,
                unsafe { Communicator::from_raw_ref(&comm) },
            ))
        }
        #[inline]
        fn alltoallv(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            sendcounts: *const c_int,
            sdispls: *const c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcounts: *const c_int,
            rdispls: *const c_int,
            recvtype: MPI_Datatype,
            comm: MPI_Comm,
        ) -> c_int {
            rmpi::Error::result_into_mpi_res((|| {
                let send_buffers =
                    buffers_from_displs(sendbuf, sendcounts, sdispls, sendtype, comm)?;
                let mut recv_buffers =
                    buffers_mut_from_displs(recvbuf, recvcounts, rdispls, recvtype, comm)?;
                <Self as MpiInterceptionLayer>::alltoallv(
                    |_rmpi_ctx, send_buffer, recv_buffer, communicator| unsafe {
                        communicator.alltoallv_with(next_f.unwrap(), send_buffer, recv_buffer)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    &send_buffers,
                    &mut recv_buffers,
                    unsafe { Communicator::from_raw_ref(&comm) },
                )
            })())
        }

        #[inline]
        fn reduce(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            recvbuf: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            op: MPI_Op,
            root: c_int,
            comm: MPI_Comm,
        ) -> c_int {
            let send_buffer =
                unsafe { TypeDynamicBufferRef::from_raw_dynamic(sendbuf, count, datatype) };
            let recv_buffer =
                unsafe { TypeDynamicBufferMut::from_raw_dynamic(recvbuf, count, datatype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::reduce(
                |_rmpi_ctx, send_buffer, recv_buffer, op, root| unsafe {
                    root.reduce_with(next_f.unwrap(), send_buffer, recv_buffer, op)
                },
                unsafe { RmpiContext::create_unchecked_ref() },
                send_buffer,
                recv_buffer,
                op.into(),
                unsafe { Communicator::from_raw_ref(&comm) }.get_process(root),
            ))
        }
        #[inline]
        fn allreduce(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            recvbuf: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            op: MPI_Op,
            comm: MPI_Comm,
        ) -> c_int {
            let send_buffer =
                unsafe { TypeDynamicBufferRef::from_raw_dynamic(sendbuf, count, datatype) };
            let recv_buffer =
                unsafe { TypeDynamicBufferMut::from_raw_dynamic(recvbuf, count, datatype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::allreduce(
                |_rmpi_ctx, send_buffer, recv_buffer, op, communicator| unsafe {
                    communicator.allreduce_with(next_f.unwrap(), send_buffer, recv_buffer, op)
                },
                unsafe { RmpiContext::create_unchecked_ref() },
                send_buffer,
                recv_buffer,
                op.into(),
                unsafe { Communicator::from_raw_ref(&comm) },
            ))
        }

        #[inline]
        fn scan(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            recvbuf: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            op: MPI_Op,
            comm: MPI_Comm,
        ) -> c_int {
            let send_buffer =
                unsafe { TypeDynamicBufferRef::from_raw_dynamic(sendbuf, count, datatype) };
            let recv_buffer =
                unsafe { TypeDynamicBufferMut::from_raw_dynamic(recvbuf, count, datatype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::scan(
                |_rmpi_ctx, send_buffer, recv_buffer, op, communicator| unsafe {
                    communicator.scan_with(next_f.unwrap(), send_buffer, recv_buffer, op)
                },
                unsafe { RmpiContext::create_unchecked_ref() },
                send_buffer,
                recv_buffer,
                op.into(),
                unsafe { Communicator::from_raw_ref(&comm) },
            ))
        }

        #[inline]
        fn scatter(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcount: c_int,
            recvtype: MPI_Datatype,
            root: c_int,
            comm: MPI_Comm,
        ) -> c_int {
            let send_buffer =
                unsafe { TypeDynamicBufferRef::from_raw_dynamic(sendbuf, sendcount, sendtype) };
            let recv_buffer =
                unsafe { TypeDynamicBufferMut::from_raw_dynamic(recvbuf, recvcount, recvtype) };
            rmpi::Error::result_into_mpi_res(<Self as MpiInterceptionLayer>::scatter(
                |_rmpi_ctx, send_buffer, recv_buffer, root| unsafe {
                    root.scatter_with(next_f.unwrap(), send_buffer, recv_buffer)
                },
                unsafe { RmpiContext::create_unchecked_ref() },
                send_buffer,
                recv_buffer,
                unsafe { Communicator::from_raw_ref(&comm) }.get_process(root),
            ))
        }
        #[inline]
        fn scatterv(
            next_f: UnsafeBox,
            sendbuf: *const c_void,
            sendcounts: *const c_int,
            displs: *const c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcount: c_int,
            recvtype: MPI_Datatype,
            root: c_int,
            comm: MPI_Comm,
        ) -> c_int {
            let recv_buffer =
                unsafe { TypeDynamicBufferMut::from_raw_dynamic(recvbuf, recvcount, recvtype) };

            rmpi::Error::result_into_mpi_res((|| {
                let send_buffers =
                    buffers_from_displs(sendbuf, sendcounts, displs, sendtype, comm)?;
                <Self as MpiInterceptionLayer>::scatterv(
                    |_rmpi_ctx, send_buffer, recv_buffer, root| unsafe {
                        root.scatterv_with(next_f.unwrap(), send_buffer, recv_buffer)
                    },
                    unsafe { RmpiContext::create_unchecked_ref() },
                    &send_buffers,
                    recv_buffer,
                    unsafe { Communicator::from_raw_ref(&comm) }.get_process(root),
                )
            })())
        }
    );
}

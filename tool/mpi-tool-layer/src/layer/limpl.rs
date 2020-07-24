use std::{os::raw::*, ptr, slice};

use rmpi::pmpi_mode as rmpi;

use mpi_sys::pmpi::*;
use self::rmpi::{
    request::{Request, RequestSlice},
    Buffer, Communicator, Group, MpiDatatype, Status,
};

use crate::{RawMpiInterceptionLayer, UnsafeBox};

use super::MpiInterceptionLayer;

impl<T> RawMpiInterceptionLayer for T
where
    T: MpiInterceptionLayer,
{
    trait_layer_function! (
        #[inline]
        fn finalize(unsafe next_f) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::finalize(
                    || rmpi::Error::from_mpi_res(unsafe{next_f.unwrap()}())
                )
            )
        }

        #[inline]
        fn wtime(unsafe next_f) -> c_double
        {
            <Self as MpiInterceptionLayer>::wtime(unsafe{next_f.unwrap()})
        }
        #[inline]
        fn wtick(unsafe next_f) -> c_double
        {
            <Self as MpiInterceptionLayer>::wtick(unsafe{next_f.unwrap()})
        }

        #[inline]
        fn barrier(unsafe next_f, comm: MPI_Comm) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::barrier(
                    |comm| unsafe{ comm.barrier_with(next_f.unwrap()) },
                    unsafe{Communicator::from_raw_ref(&comm)}
                )
            )
        }

        #[inline]
        fn group_incl(
            unsafe next_f,
            group: MPI_Group,
            n: c_int,
            ranks: *const c_int,
            newgroup: *mut MPI_Group,
        ) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::group_incl(
                    |group, ranks|
                        unsafe { group.incl_with(next_f.unwrap(), ranks) },
                    unsafe{Group::from_raw_ref(&group)},
                    unsafe{slice::from_raw_parts(ranks, n as usize)}
                ).map(|newgroup_output|{
                    unsafe{newgroup.write(newgroup_output.into_raw())};
                })
            )
        }
        #[inline]
        fn group_free(unsafe next_f, group: *mut MPI_Group) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::group_free(
                    |group|
                        unsafe{group.free_with(next_f.unwrap())},
                    unsafe{Group::from_raw(*group)}
                )
            )
        }

        #[inline]
        fn comm_size(unsafe next_f, comm: MPI_Comm, size: *mut c_int) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::comm_size(
                    |comm|
                        unsafe{comm.size_with(next_f.unwrap())},
                    unsafe{Communicator::from_raw_ref(&comm)},
                ).map(|size_out|{
                    unsafe{size.write(size_out)};
                })
            )
        }
        #[inline]
        fn comm_rank(unsafe next_f, comm: MPI_Comm, rank: *mut c_int) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::comm_rank(
                    |comm|
                        unsafe{comm.current_rank_with(next_f.unwrap())},
                    unsafe{Communicator::from_raw_ref(&comm)},
                ).map(|rank_out|{
                    unsafe{rank.write(rank_out)};
                })
            )
        }
        #[inline]
        fn comm_create(
            unsafe next_f,
            comm: MPI_Comm,
            group: MPI_Group,
            newcomm: *mut MPI_Comm,
        ) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::comm_create(
                    |comm, group| unsafe {
                        comm.create_subset_with(next_f.unwrap(), group)
                    },
                    unsafe{Communicator::from_raw_ref(&comm)},
                    unsafe{Group::from_raw_ref(&group)},
                ).map(|newcomm_output|{
                    unsafe{newcomm.write(newcomm_output.into_raw())};
                })
            )
        }
        #[inline]
        fn comm_free(unsafe next_f, comm: *mut MPI_Comm) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::comm_free(
                    |comm|
                        unsafe{comm.free_with(next_f.unwrap())},
                    unsafe{Communicator::from_raw(*comm)}
                )
            )
        }

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
                            |buf, dest, tag| {
                                unsafe{dest.send_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw_ref(&comm)}.get_process(dest),
                            tag.into()
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
                            |buf, dest, tag| {
                                unsafe{dest.bsend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw_ref(&comm)}.get_process(dest),
                            tag.into()
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
                            |buf, dest, tag| {
                                unsafe{dest.ssend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw_ref(&comm)}.get_process(dest),
                            tag.into()
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
                            |buf, dest, tag| {
                                unsafe{dest.rsend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw_ref(&comm)}.get_process(dest),
                            tag.into()
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
                            |buf, dest, tag| {
                                unsafe{dest.isend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw_ref(&comm)}.get_process(dest),
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
                            |buf, dest, tag| {
                                unsafe{dest.ibsend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw_ref(&comm)}.get_process(dest),
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
                            |buf, dest, tag| {
                                unsafe{dest.issend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw_ref(&comm)}.get_process(dest),
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
                            |buf, dest, tag| {
                                unsafe{dest.irsend_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw_ref(&comm)}.get_process(dest),
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
                            |buf, dest, tag| {
                                unsafe{dest.recv_with(next_f.unwrap(), buf, tag)}
                            },
                            buffer,
                            unsafe{Communicator::from_raw_ref(&comm)}.get_process(source),
                            tag.into()
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
                        RequestSlice::from_raw_mut(
                            slice::from_raw_parts_mut(array_of_requests, count as usize)
                        )
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
        fn waitall(
            unsafe next_f,
            count: c_int,
            array_of_requests: *mut MPI_Request,
            array_of_statuses: *mut MPI_Status,
        ) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::waitall(
                    |req_slc, status_slc| {
                        unsafe {
                            req_slc.waitall_with(next_f.unwrap(), status_slc)
                        }
                    },
                    unsafe{
                        RequestSlice::from_raw_mut(
                            slice::from_raw_parts_mut(array_of_requests, count as usize)
                        )
                    },
                    Status::from_raw_slice_mut(array_of_statuses, count as usize)
                )
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
        fn testany(
            unsafe next_f,
            count: c_int,
            array_of_requests: *mut MPI_Request,
            indx: *mut c_int,
            flag: *mut c_int,
            status: *mut MPI_Status,
        ) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::testany(
                    |req_slc| {
                        unsafe {
                            req_slc.testany_with(next_f.unwrap())
                        }
                    },
                    unsafe{
                        RequestSlice::from_raw_mut(
                            slice::from_raw_parts_mut(array_of_requests, count as usize)
                        )
                    }
                ).map(|opt_out|{
                    match opt_out {
                        Some((out_indx, out_status)) => {
                            unsafe {
                                *flag = 1;
                                indx.write(out_indx as c_int);
                                status.write(out_status.into_raw());
                            }
                        }
                        None => unsafe{*flag = 0}
                    }
                })
            )
        }

        #[inline]
        fn request_free(unsafe next_f, request: *mut MPI_Request) -> c_int
        {
            rmpi::Error::result_into_mpi_res(
                <Self as MpiInterceptionLayer>::request_free(
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
        fn bcast(
            unsafe next_f,
            buffer: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            root: c_int,
            comm: MPI_Comm,
        ) -> c_int
        {
            unsafe_with_buf_mut!(
                (buffer,count,datatype) => buffer => {
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::bcast(
                            |buf, root| {
                                unsafe{root.bcast_with(next_f.unwrap(), buf)}
                            },
                            buffer, unsafe{Communicator::from_raw_ref(&comm)}.get_process(root)
                        )
                    )
                }
            )
        }

        #[inline]
        fn gather(
            unsafe next_f,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcount: c_int,
            recvtype: MPI_Datatype,
            root: c_int,
            comm: MPI_Comm,
        ) -> c_int
        {
            unsafe_with_buf!(
                (sendbuf, sendcount, sendtype) => send_buffer => {
                    unsafe_with_buf_mut!(
                        (recvbuf, recvcount, recvtype) => recv_buffer => {
                            rmpi::Error::result_into_mpi_res(
                                <Self as MpiInterceptionLayer>::gather(
                                    |send_buffer, recv_buffer, root| {
                                        unsafe {
                                            root.gather_with(next_f.unwrap(), send_buffer, recv_buffer)
                                        }
                                    },
                                    send_buffer, recv_buffer,
                                    unsafe{Communicator::from_raw_ref(&comm)}.get_process(root)
                                )
                            )
                        }
                    )
                }
            )
        }
        #[inline]
        fn gatherv(
            unsafe next_f,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcounts: *const c_int,
            displs: *const c_int,
            recvtype: MPI_Datatype,
            root: c_int,
            comm: MPI_Comm,
        ) -> c_int
        {
            unsafe_with_buf!(
                (sendbuf, sendcount, sendtype) => send_buffer => {
                    define_datatype!(
                        type RecvElem = recvtype;
                        {
                            let recvbuf = recvbuf as *mut RecvElem;
                            let mut recv_buffers = vec![];
                            for recv_rank in 0..unsafe{Communicator::from_raw_ref(&comm)}.size().unwrap(){
                                let recv_buffer_part = unsafe {
                                    <[RecvElem] as Buffer>::from_raw_mut(
                                        recvbuf.add(
                                            *displs.add(recv_rank as usize) as usize
                                        ) as *mut c_void,
                                        *recvcounts.add(recv_rank as usize)
                                    )
                                };
                                recv_buffers.push(recv_buffer_part);
                            };
                            rmpi::Error::result_into_mpi_res(
                                <Self as MpiInterceptionLayer>::gatherv(
                                    |send_buffer, recv_buffer, root| {
                                        unsafe {
                                            root.gatherv_with(next_f.unwrap(), send_buffer, recv_buffer)
                                        }
                                    },
                                    send_buffer, &mut recv_buffers,
                                    unsafe{Communicator::from_raw_ref(&comm)}.get_process(root)
                                )
                            )
                        }
                    )
                }
            )
        }

        #[inline]
        fn allgather(
            unsafe next_f,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcount: c_int,
            recvtype: MPI_Datatype,
            comm: MPI_Comm,
        ) -> c_int
        {
            unsafe_with_buf!(
                (sendbuf, sendcount, sendtype) => send_buffer => {
                    unsafe_with_buf_mut!(
                        (recvbuf, recvcount, recvtype) => recv_buffer => {
                            rmpi::Error::result_into_mpi_res(
                                <Self as MpiInterceptionLayer>::allgather(
                                    |send_buffer, recv_buffer, communicator| {
                                        unsafe {
                                            communicator.allgather_with(next_f.unwrap(), send_buffer, recv_buffer)
                                        }
                                    },
                                    send_buffer, recv_buffer,
                                    unsafe{Communicator::from_raw_ref(&comm)}
                                )
                            )
                        }
                    )
                }
            )
        }
        #[inline]
        fn allgatherv(
            unsafe next_f,
            sendbuf: *const c_void,
            sendcount: c_int,
            sendtype: MPI_Datatype,
            recvbuf: *mut c_void,
            recvcounts: *const c_int,
            displs: *const c_int,
            recvtype: MPI_Datatype,
            comm: MPI_Comm,
        ) -> c_int
        {
            unsafe_with_buf!(
                (sendbuf, sendcount, sendtype) => send_buffer => {
                    define_datatype!(
                        type RecvElem = recvtype;
                        {
                            let recvbuf = recvbuf as *mut RecvElem;
                            let mut recv_buffers = vec![];
                            for recv_rank in 
                                    0..unsafe{Communicator::from_raw_ref(&comm)}.size().unwrap(){
                                let recv_buffer_part = unsafe {
                                    <[RecvElem] as Buffer>::from_raw_mut(
                                        recvbuf.add(
                                            *displs.add(recv_rank as usize) as usize
                                        ) as *mut c_void,
                                        *recvcounts.add(recv_rank as usize)
                                    )
                                };
                                recv_buffers.push(recv_buffer_part);
                            };
                            rmpi::Error::result_into_mpi_res(
                                <Self as MpiInterceptionLayer>::allgatherv(
                                    |send_buffer, recv_buffer, communicator| {
                                        unsafe {
                                            communicator.allgatherv_with(
                                                next_f.unwrap(), send_buffer, recv_buffer
                                            )
                                        }
                                    },
                                    send_buffer, &mut recv_buffers,
                                    unsafe{Communicator::from_raw_ref(&comm)}
                                )
                            )
                        }
                    )
                }
            )
        }

        #[inline]
        fn reduce(
            unsafe next_f,
            sendbuf: *const c_void,
            recvbuf: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            op: MPI_Op,
            root: c_int,
            comm: MPI_Comm,
        ) -> c_int
        {
            define_datatype!(
                type Elem = datatype;
                {
                    let send_buffer = unsafe { <[Elem] as Buffer>::from_raw(sendbuf, count) };
                    let recv_buffer = unsafe { <Elem as Buffer>::from_raw_mut(recvbuf, 1) };
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::reduce(
                            |send_buffer, recv_buffer, op, root| {
                                unsafe {
                                    root.reduce_with(
                                        next_f.unwrap(), send_buffer, recv_buffer, op
                                    )
                                }
                            },
                            send_buffer, recv_buffer, op.into(),
                            unsafe{Communicator::from_raw_ref(&comm)}.get_process(root)
                        )
                    )
                }
            )
        }
        #[inline]
        fn allreduce(
            unsafe next_f,
            sendbuf: *const c_void,
            recvbuf: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            op: MPI_Op,
            comm: MPI_Comm,
        ) -> c_int
        {
            define_datatype!(
                type Elem = datatype;
                {
                    let send_buffer = unsafe { <[Elem] as Buffer>::from_raw(sendbuf, count) };
                    let recv_buffer = unsafe { <Elem as Buffer>::from_raw_mut(recvbuf, 1) };
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::allreduce(
                            |send_buffer, recv_buffer, op, communicator| {
                                unsafe {
                                    communicator.allreduce_with(
                                        next_f.unwrap(), send_buffer, recv_buffer, op
                                    )
                                }
                            },
                            send_buffer, recv_buffer, op.into(),
                            unsafe{Communicator::from_raw_ref(&comm)}
                        )
                    )
                }
            )
        }

        #[inline]
        fn scan(
            unsafe next_f,
            sendbuf: *const c_void,
            recvbuf: *mut c_void,
            count: c_int,
            datatype: MPI_Datatype,
            op: MPI_Op,
            comm: MPI_Comm,
        ) -> c_int
        {
            define_datatype!(
                type Elem = datatype;
                {
                    let send_buffer = unsafe { <[Elem] as Buffer>::from_raw(sendbuf, count) };
                    let recv_buffer = unsafe { <Elem as Buffer>::from_raw_mut(recvbuf, 1) };
                    rmpi::Error::result_into_mpi_res(
                        <Self as MpiInterceptionLayer>::scan(
                            |send_buffer, recv_buffer, op, communicator| {
                                unsafe {
                                    communicator.scan_with(
                                        next_f.unwrap(), send_buffer, recv_buffer, op
                                    )
                                }
                            },
                            send_buffer, recv_buffer, op.into(),
                            unsafe{Communicator::from_raw_ref(&comm)}
                        )
                    )
                }
            )
        }
    );
}

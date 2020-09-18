use std::os::raw::*;

use mpi_sys::pmpi::*;

use crate::UnsafeBox;

pub trait RawMpiInterceptionLayer {
    #[inline]
    fn send<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_void, c_int, MPI_Datatype, c_int, c_int, MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm) }
    }
    #[inline]
    fn recv<F>(
        next_f: UnsafeBox<F>,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: MPI_Comm,
        status: *mut MPI_Status,
    ) -> c_int
    where
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
        unsafe { next_f.unwrap()(buf, count, datatype, source, tag, comm, status) }
    }
    #[inline]
    fn get_count<F>(
        next_f: UnsafeBox<F>,
        status: *const MPI_Status,
        datatype: MPI_Datatype,
        count: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(*const MPI_Status, MPI_Datatype, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(status, datatype, count) }
    }
    #[inline]
    fn bsend<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_void, c_int, MPI_Datatype, c_int, c_int, MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm) }
    }
    #[inline]
    fn ssend<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_void, c_int, MPI_Datatype, c_int, c_int, MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm) }
    }
    #[inline]
    fn rsend<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_void, c_int, MPI_Datatype, c_int, c_int, MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm) }
    }
    #[inline]
    fn buffer_attach<F>(next_f: UnsafeBox<F>, buffer: *mut c_void, size: c_int) -> c_int
    where
        F: FnOnce(*mut c_void, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(buffer, size) }
    }
    #[inline]
    fn buffer_detach<F>(next_f: UnsafeBox<F>, buffer_addr: *mut c_void, size: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_void, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(buffer_addr, size) }
    }
    #[inline]
    fn isend<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn ibsend<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn issend<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn irsend<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn irecv<F>(
        next_f: UnsafeBox<F>,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            c_int,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(buf, count, datatype, source, tag, comm, request) }
    }
    #[inline]
    fn wait<F>(next_f: UnsafeBox<F>, request: *mut MPI_Request, status: *mut MPI_Status) -> c_int
    where
        F: FnOnce(*mut MPI_Request, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(request, status) }
    }
    #[inline]
    fn test<F>(
        next_f: UnsafeBox<F>,
        request: *mut MPI_Request,
        flag: *mut c_int,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(*mut MPI_Request, *mut c_int, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(request, flag, status) }
    }
    #[inline]
    fn request_free<F>(next_f: UnsafeBox<F>, request: *mut MPI_Request) -> c_int
    where
        F: FnOnce(*mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(request) }
    }
    #[inline]
    fn waitany<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        array_of_requests: *mut MPI_Request,
        indx: *mut c_int,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut MPI_Request, *mut c_int, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(count, array_of_requests, indx, status) }
    }
    #[inline]
    fn testany<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        array_of_requests: *mut MPI_Request,
        indx: *mut c_int,
        flag: *mut c_int,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut MPI_Request, *mut c_int, *mut c_int, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(count, array_of_requests, indx, flag, status) }
    }
    #[inline]
    fn waitall<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        array_of_requests: *mut MPI_Request,
        array_of_statuses: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut MPI_Request, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(count, array_of_requests, array_of_statuses) }
    }
    #[inline]
    fn testall<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        array_of_requests: *mut MPI_Request,
        flag: *mut c_int,
        array_of_statuses: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut MPI_Request, *mut c_int, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(count, array_of_requests, flag, array_of_statuses) }
    }
    #[inline]
    fn waitsome<F>(
        next_f: UnsafeBox<F>,
        incount: c_int,
        array_of_requests: *mut MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut MPI_Request, *mut c_int, *mut c_int, *mut MPI_Status) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                incount,
                array_of_requests,
                outcount,
                array_of_indices,
                array_of_statuses,
            )
        }
    }
    #[inline]
    fn testsome<F>(
        next_f: UnsafeBox<F>,
        incount: c_int,
        array_of_requests: *mut MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut MPI_Request, *mut c_int, *mut c_int, *mut MPI_Status) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                incount,
                array_of_requests,
                outcount,
                array_of_indices,
                array_of_statuses,
            )
        }
    }
    #[inline]
    fn iprobe<F>(
        next_f: UnsafeBox<F>,
        source: c_int,
        tag: c_int,
        comm: MPI_Comm,
        flag: *mut c_int,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, MPI_Comm, *mut c_int, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(source, tag, comm, flag, status) }
    }
    #[inline]
    fn probe<F>(
        next_f: UnsafeBox<F>,
        source: c_int,
        tag: c_int,
        comm: MPI_Comm,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, MPI_Comm, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(source, tag, comm, status) }
    }
    #[inline]
    fn cancel<F>(next_f: UnsafeBox<F>, request: *mut MPI_Request) -> c_int
    where
        F: FnOnce(*mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(request) }
    }
    #[inline]
    fn test_cancelled<F>(next_f: UnsafeBox<F>, status: *const MPI_Status, flag: *mut c_int) -> c_int
    where
        F: FnOnce(*const MPI_Status, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(status, flag) }
    }
    #[inline]
    fn send_init<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn bsend_init<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn ssend_init<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn rsend_init<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn recv_init<F>(
        next_f: UnsafeBox<F>,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            c_int,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(buf, count, datatype, source, tag, comm, request) }
    }
    #[inline]
    fn start<F>(next_f: UnsafeBox<F>, request: *mut MPI_Request) -> c_int
    where
        F: FnOnce(*mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(request) }
    }
    #[inline]
    fn startall<F>(next_f: UnsafeBox<F>, count: c_int, array_of_requests: *mut MPI_Request) -> c_int
    where
        F: FnOnce(c_int, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(count, array_of_requests) }
    }
    #[inline]
    fn sendrecv<F>(
        next_f: UnsafeBox<F>,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            c_int,
            c_int,
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            c_int,
            MPI_Comm,
            *mut MPI_Status,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, dest, sendtag, recvbuf, recvcount, recvtype, source,
                recvtag, comm, status,
            )
        }
    }
    #[inline]
    fn sendrecv_replace<F>(
        next_f: UnsafeBox<F>,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        sendtag: c_int,
        source: c_int,
        recvtag: c_int,
        comm: MPI_Comm,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            c_int,
            c_int,
            c_int,
            MPI_Comm,
            *mut MPI_Status,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                buf, count, datatype, dest, sendtag, source, recvtag, comm, status,
            )
        }
    }
    #[inline]
    fn type_contiguous<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(count, oldtype, newtype) }
    }
    #[inline]
    fn type_vector<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        blocklength: c_int,
        stride: c_int,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, c_int, MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(count, blocklength, stride, oldtype, newtype) }
    }
    #[inline]
    fn type_hvector<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        blocklength: c_int,
        stride: MPI_Aint,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, MPI_Aint, MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(count, blocklength, stride, oldtype, newtype) }
    }
    #[inline]
    fn type_indexed<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const c_int,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, *const c_int, *const c_int, MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                count,
                array_of_blocklengths,
                array_of_displacements,
                oldtype,
                newtype,
            )
        }
    }
    #[inline]
    fn type_hindexed<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut MPI_Aint,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut c_int, *mut MPI_Aint, MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                count,
                array_of_blocklengths,
                array_of_displacements,
                oldtype,
                newtype,
            )
        }
    }
    #[inline]
    fn type_struct<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut MPI_Aint,
        array_of_types: *mut MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut c_int, *mut MPI_Aint, *mut MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                count,
                array_of_blocklengths,
                array_of_displacements,
                array_of_types,
                newtype,
            )
        }
    }
    #[inline]
    fn address<F>(next_f: UnsafeBox<F>, location: *mut c_void, address: *mut MPI_Aint) -> c_int
    where
        F: FnOnce(*mut c_void, *mut MPI_Aint) -> c_int,
    {
        unsafe { next_f.unwrap()(location, address) }
    }
    #[inline]
    fn type_extent<F>(next_f: UnsafeBox<F>, datatype: MPI_Datatype, extent: *mut MPI_Aint) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut MPI_Aint) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, extent) }
    }
    #[inline]
    fn type_size<F>(next_f: UnsafeBox<F>, datatype: MPI_Datatype, size: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, size) }
    }
    #[inline]
    fn type_lb<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        displacement: *mut MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut MPI_Aint) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, displacement) }
    }
    #[inline]
    fn type_ub<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        displacement: *mut MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut MPI_Aint) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, displacement) }
    }
    #[inline]
    fn type_commit<F>(next_f: UnsafeBox<F>, datatype: *mut MPI_Datatype) -> c_int
    where
        F: FnOnce(*mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype) }
    }
    #[inline]
    fn type_free<F>(next_f: UnsafeBox<F>, datatype: *mut MPI_Datatype) -> c_int
    where
        F: FnOnce(*mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype) }
    }
    #[inline]
    fn get_elements<F>(
        next_f: UnsafeBox<F>,
        status: *const MPI_Status,
        datatype: MPI_Datatype,
        count: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(*const MPI_Status, MPI_Datatype, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(status, datatype, count) }
    }
    #[inline]
    fn pack<F>(
        next_f: UnsafeBox<F>,
        inbuf: *const c_void,
        incount: c_int,
        datatype: MPI_Datatype,
        outbuf: *mut c_void,
        outsize: c_int,
        position: *mut c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            *mut c_int,
            MPI_Comm,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(inbuf, incount, datatype, outbuf, outsize, position, comm) }
    }
    #[inline]
    fn unpack<F>(
        next_f: UnsafeBox<F>,
        inbuf: *const c_void,
        insize: c_int,
        position: *mut c_int,
        outbuf: *mut c_void,
        outcount: c_int,
        datatype: MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            *mut c_int,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Comm,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(inbuf, insize, position, outbuf, outcount, datatype, comm) }
    }
    #[inline]
    fn pack_size<F>(
        next_f: UnsafeBox<F>,
        incount: c_int,
        datatype: MPI_Datatype,
        comm: MPI_Comm,
        size: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(c_int, MPI_Datatype, MPI_Comm, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(incount, datatype, comm, size) }
    }
    #[inline]
    fn barrier<F>(next_f: UnsafeBox<F>, comm: MPI_Comm) -> c_int
    where
        F: FnOnce(MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm) }
    }
    #[inline]
    fn bcast<F>(
        next_f: UnsafeBox<F>,
        buffer: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        root: c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*mut c_void, c_int, MPI_Datatype, c_int, MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(buffer, count, datatype, root, comm) }
    }
    #[inline]
    fn gather<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        root: c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm,
            )
        }
    }
    #[inline]
    fn gatherv<F>(
        next_f: UnsafeBox<F>,
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
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, root, comm,
            )
        }
    }
    #[inline]
    fn scatter<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        root: c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm,
            )
        }
    }
    #[inline]
    fn scatterv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        displs: *const c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        root: c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Comm,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, displs, sendtype, recvbuf, recvcount, recvtype, root, comm,
            )
        }
    }
    #[inline]
    fn allgather<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
            )
        }
    }
    #[inline]
    fn allgatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm,
            )
        }
    }
    #[inline]
    fn alltoall<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
            )
        }
    }
    #[inline]
    fn alltoallv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, sdispls, sendtype, recvbuf, recvcounts, rdispls, recvtype,
                comm,
            )
        }
    }
    #[inline]
    fn alltoallw<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtypes: *const MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtypes: *const MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            *const MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            *const MPI_Datatype,
            MPI_Comm,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, sdispls, sendtypes, recvbuf, recvcounts, rdispls, recvtypes,
                comm,
            )
        }
    }
    #[inline]
    fn exscan<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_void, *mut c_void, c_int, MPI_Datatype, MPI_Op, MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm) }
    }
    #[inline]
    fn reduce<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        root: c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Op,
            c_int,
            MPI_Comm,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, root, comm) }
    }
    #[inline]
    fn op_create<F>(
        next_f: UnsafeBox<F>,
        user_fn: MPI_User_function,
        commute: c_int,
        op: *mut MPI_Op,
    ) -> c_int
    where
        F: FnOnce(MPI_User_function, c_int, *mut MPI_Op) -> c_int,
    {
        unsafe { next_f.unwrap()(user_fn, commute, op) }
    }
    #[inline]
    fn op_free<F>(next_f: UnsafeBox<F>, op: *mut MPI_Op) -> c_int
    where
        F: FnOnce(*mut MPI_Op) -> c_int,
    {
        unsafe { next_f.unwrap()(op) }
    }
    #[inline]
    fn allreduce<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_void, *mut c_void, c_int, MPI_Datatype, MPI_Op, MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm) }
    }
    #[inline]
    fn reduce_scatter<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            *const c_int,
            MPI_Datatype,
            MPI_Op,
            MPI_Comm,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, recvcounts, datatype, op, comm) }
    }
    #[inline]
    fn scan<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_void, *mut c_void, c_int, MPI_Datatype, MPI_Op, MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm) }
    }
    #[inline]
    fn group_size<F>(next_f: UnsafeBox<F>, group: MPI_Group, size: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Group, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(group, size) }
    }
    #[inline]
    fn group_rank<F>(next_f: UnsafeBox<F>, group: MPI_Group, rank: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Group, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(group, rank) }
    }
    #[inline]
    fn group_translate_ranks<F>(
        next_f: UnsafeBox<F>,
        group1: MPI_Group,
        n: c_int,
        ranks1: *const c_int,
        group2: MPI_Group,
        ranks2: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Group, c_int, *const c_int, MPI_Group, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(group1, n, ranks1, group2, ranks2) }
    }
    #[inline]
    fn group_compare<F>(
        next_f: UnsafeBox<F>,
        group1: MPI_Group,
        group2: MPI_Group,
        result: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Group, MPI_Group, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(group1, group2, result) }
    }
    #[inline]
    fn comm_group<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, group: *mut MPI_Group) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, group) }
    }
    #[inline]
    fn group_union<F>(
        next_f: UnsafeBox<F>,
        group1: MPI_Group,
        group2: MPI_Group,
        newgroup: *mut MPI_Group,
    ) -> c_int
    where
        F: FnOnce(MPI_Group, MPI_Group, *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(group1, group2, newgroup) }
    }
    #[inline]
    fn group_intersection<F>(
        next_f: UnsafeBox<F>,
        group1: MPI_Group,
        group2: MPI_Group,
        newgroup: *mut MPI_Group,
    ) -> c_int
    where
        F: FnOnce(MPI_Group, MPI_Group, *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(group1, group2, newgroup) }
    }
    #[inline]
    fn group_difference<F>(
        next_f: UnsafeBox<F>,
        group1: MPI_Group,
        group2: MPI_Group,
        newgroup: *mut MPI_Group,
    ) -> c_int
    where
        F: FnOnce(MPI_Group, MPI_Group, *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(group1, group2, newgroup) }
    }
    #[inline]
    fn group_incl<F>(
        next_f: UnsafeBox<F>,
        group: MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut MPI_Group,
    ) -> c_int
    where
        F: FnOnce(MPI_Group, c_int, *const c_int, *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(group, n, ranks, newgroup) }
    }
    #[inline]
    fn group_excl<F>(
        next_f: UnsafeBox<F>,
        group: MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut MPI_Group,
    ) -> c_int
    where
        F: FnOnce(MPI_Group, c_int, *const c_int, *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(group, n, ranks, newgroup) }
    }
    #[inline]
    fn group_range_incl<F>(
        next_f: UnsafeBox<F>,
        group: MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3usize],
        newgroup: *mut MPI_Group,
    ) -> c_int
    where
        F: FnOnce(MPI_Group, c_int, *mut [c_int; 3usize], *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(group, n, ranges, newgroup) }
    }
    #[inline]
    fn group_range_excl<F>(
        next_f: UnsafeBox<F>,
        group: MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3usize],
        newgroup: *mut MPI_Group,
    ) -> c_int
    where
        F: FnOnce(MPI_Group, c_int, *mut [c_int; 3usize], *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(group, n, ranges, newgroup) }
    }
    #[inline]
    fn group_free<F>(next_f: UnsafeBox<F>, group: *mut MPI_Group) -> c_int
    where
        F: FnOnce(*mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(group) }
    }
    #[inline]
    fn comm_size<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, size: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, size) }
    }
    #[inline]
    fn comm_rank<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, rank: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, rank) }
    }
    #[inline]
    fn comm_compare<F>(
        next_f: UnsafeBox<F>,
        comm1: MPI_Comm,
        comm2: MPI_Comm,
        result: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, MPI_Comm, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm1, comm2, result) }
    }
    #[inline]
    fn comm_dup<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, newcomm: *mut MPI_Comm) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, newcomm) }
    }
    #[inline]
    fn comm_dup_with_info<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        info: MPI_Info,
        newcomm: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, MPI_Info, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, info, newcomm) }
    }
    #[inline]
    fn comm_create<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        group: MPI_Group,
        newcomm: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, MPI_Group, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, group, newcomm) }
    }
    #[inline]
    fn comm_split<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        color: c_int,
        key: c_int,
        newcomm: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, c_int, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, color, key, newcomm) }
    }
    #[inline]
    fn comm_free<F>(next_f: UnsafeBox<F>, comm: *mut MPI_Comm) -> c_int
    where
        F: FnOnce(*mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm) }
    }
    #[inline]
    fn comm_test_inter<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, flag: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, flag) }
    }
    #[inline]
    fn comm_remote_size<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, size: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, size) }
    }
    #[inline]
    fn comm_remote_group<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, group: *mut MPI_Group) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, group) }
    }
    #[inline]
    fn intercomm_create<F>(
        next_f: UnsafeBox<F>,
        local_comm: MPI_Comm,
        local_leader: c_int,
        peer_comm: MPI_Comm,
        remote_leader: c_int,
        tag: c_int,
        newintercomm: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, MPI_Comm, c_int, c_int, *mut MPI_Comm) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                local_comm,
                local_leader,
                peer_comm,
                remote_leader,
                tag,
                newintercomm,
            )
        }
    }
    #[inline]
    fn intercomm_merge<F>(
        next_f: UnsafeBox<F>,
        intercomm: MPI_Comm,
        high: c_int,
        newintracomm: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(intercomm, high, newintracomm) }
    }
    #[inline]
    fn keyval_create<F>(
        next_f: UnsafeBox<F>,
        copy_fn: MPI_Copy_function,
        delete_fn: MPI_Delete_function,
        keyval: *mut c_int,
        extra_state: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(MPI_Copy_function, MPI_Delete_function, *mut c_int, *mut c_void) -> c_int,
    {
        unsafe { next_f.unwrap()(copy_fn, delete_fn, keyval, extra_state) }
    }
    #[inline]
    fn keyval_free<F>(next_f: UnsafeBox<F>, keyval: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(keyval) }
    }
    #[inline]
    fn attr_put<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        keyval: c_int,
        attribute_val: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *mut c_void) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, keyval, attribute_val) }
    }
    #[inline]
    fn attr_get<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *mut c_void, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, keyval, attribute_val, flag) }
    }
    #[inline]
    fn attr_delete<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, keyval: c_int) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, keyval) }
    }
    #[inline]
    fn topo_test<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, status: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, status) }
    }
    #[inline]
    fn cart_create<F>(
        next_f: UnsafeBox<F>,
        comm_old: MPI_Comm,
        ndims: c_int,
        dims: *const c_int,
        periods: *const c_int,
        reorder: c_int,
        comm_cart: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *const c_int, *const c_int, c_int, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm_old, ndims, dims, periods, reorder, comm_cart) }
    }
    #[inline]
    fn dims_create<F>(next_f: UnsafeBox<F>, nnodes: c_int, ndims: c_int, dims: *mut c_int) -> c_int
    where
        F: FnOnce(c_int, c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(nnodes, ndims, dims) }
    }
    #[inline]
    fn graph_create<F>(
        next_f: UnsafeBox<F>,
        comm_old: MPI_Comm,
        nnodes: c_int,
        indx: *const c_int,
        edges: *const c_int,
        reorder: c_int,
        comm_graph: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *const c_int, *const c_int, c_int, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm_old, nnodes, indx, edges, reorder, comm_graph) }
    }
    #[inline]
    fn graphdims_get<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        nnodes: *mut c_int,
        nedges: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, nnodes, nedges) }
    }
    #[inline]
    fn graph_get<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        maxindex: c_int,
        maxedges: c_int,
        indx: *mut c_int,
        edges: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, c_int, *mut c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, maxindex, maxedges, indx, edges) }
    }
    #[inline]
    fn cartdim_get<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, ndims: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, ndims) }
    }
    #[inline]
    fn cart_get<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        maxdims: c_int,
        dims: *mut c_int,
        periods: *mut c_int,
        coords: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *mut c_int, *mut c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, maxdims, dims, periods, coords) }
    }
    #[inline]
    fn cart_rank<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        coords: *const c_int,
        rank: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, *const c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, coords, rank) }
    }
    #[inline]
    fn cart_coords<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        rank: c_int,
        maxdims: c_int,
        coords: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, rank, maxdims, coords) }
    }
    #[inline]
    fn graph_neighbors_count<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        rank: c_int,
        nneighbors: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, rank, nneighbors) }
    }
    #[inline]
    fn graph_neighbors<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        rank: c_int,
        maxneighbors: c_int,
        neighbors: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, rank, maxneighbors, neighbors) }
    }
    #[inline]
    fn cart_shift<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        direction: c_int,
        disp: c_int,
        rank_source: *mut c_int,
        rank_dest: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, c_int, *mut c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, direction, disp, rank_source, rank_dest) }
    }
    #[inline]
    fn cart_sub<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        remain_dims: *const c_int,
        newcomm: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, *const c_int, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, remain_dims, newcomm) }
    }
    #[inline]
    fn cart_map<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        ndims: c_int,
        dims: *const c_int,
        periods: *const c_int,
        newrank: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *const c_int, *const c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, ndims, dims, periods, newrank) }
    }
    #[inline]
    fn graph_map<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        nnodes: c_int,
        indx: *const c_int,
        edges: *const c_int,
        newrank: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *const c_int, *const c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, nnodes, indx, edges, newrank) }
    }
    #[inline]
    fn get_processor_name<F>(
        next_f: UnsafeBox<F>,
        name: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(*mut c_char, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(name, resultlen) }
    }
    #[inline]
    fn get_version<F>(next_f: UnsafeBox<F>, version: *mut c_int, subversion: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(version, subversion) }
    }
    #[inline]
    fn get_library_version<F>(
        next_f: UnsafeBox<F>,
        version: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(*mut c_char, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(version, resultlen) }
    }
    #[inline]
    fn errhandler_create<F>(
        next_f: UnsafeBox<F>,
        function: MPI_Handler_function,
        errhandler: *mut MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_Handler_function, *mut MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(function, errhandler) }
    }
    #[inline]
    fn errhandler_set<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, errhandler: MPI_Errhandler) -> c_int
    where
        F: FnOnce(MPI_Comm, MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, errhandler) }
    }
    #[inline]
    fn errhandler_get<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        errhandler: *mut MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, errhandler) }
    }
    #[inline]
    fn errhandler_free<F>(next_f: UnsafeBox<F>, errhandler: *mut MPI_Errhandler) -> c_int
    where
        F: FnOnce(*mut MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(errhandler) }
    }
    #[inline]
    fn error_string<F>(
        next_f: UnsafeBox<F>,
        errorcode: c_int,
        string: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut c_char, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(errorcode, string, resultlen) }
    }
    #[inline]
    fn error_class<F>(next_f: UnsafeBox<F>, errorcode: c_int, errorclass: *mut c_int) -> c_int
    where
        F: FnOnce(c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(errorcode, errorclass) }
    }
    #[inline]
    fn wtime<F>(next_f: UnsafeBox<F>) -> f64
    where
        F: FnOnce() -> f64,
    {
        unsafe { next_f.unwrap()() }
    }
    #[inline]
    fn wtick<F>(next_f: UnsafeBox<F>) -> f64
    where
        F: FnOnce() -> f64,
    {
        unsafe { next_f.unwrap()() }
    }
    #[inline]
    fn init<F>(next_f: UnsafeBox<F>, argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int
    where
        F: FnOnce(*mut c_int, *mut *mut *mut c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(argc, argv) }
    }
    #[inline]
    fn finalize<F>(next_f: UnsafeBox<F>) -> c_int
    where
        F: FnOnce() -> c_int,
    {
        unsafe { next_f.unwrap()() }
    }
    #[inline]
    fn initialized<F>(next_f: UnsafeBox<F>, flag: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(flag) }
    }
    #[inline]
    fn abort<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, errorcode: c_int) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, errorcode) }
    }
    #[inline]
    fn pcontrol<F>(next_f: UnsafeBox<F>, level: c_int) -> c_int
    where
        F: FnOnce(c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(level) }
    }
    #[inline]
    fn close_port<F>(next_f: UnsafeBox<F>, port_name: *const c_char) -> c_int
    where
        F: FnOnce(*const c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(port_name) }
    }
    #[inline]
    fn comm_accept<F>(
        next_f: UnsafeBox<F>,
        port_name: *const c_char,
        info: MPI_Info,
        root: c_int,
        comm: MPI_Comm,
        newcomm: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_char, MPI_Info, c_int, MPI_Comm, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(port_name, info, root, comm, newcomm) }
    }
    #[inline]
    fn comm_connect<F>(
        next_f: UnsafeBox<F>,
        port_name: *const c_char,
        info: MPI_Info,
        root: c_int,
        comm: MPI_Comm,
        newcomm: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_char, MPI_Info, c_int, MPI_Comm, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(port_name, info, root, comm, newcomm) }
    }
    #[inline]
    fn comm_disconnect<F>(next_f: UnsafeBox<F>, comm: *mut MPI_Comm) -> c_int
    where
        F: FnOnce(*mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm) }
    }
    #[inline]
    fn comm_get_parent<F>(next_f: UnsafeBox<F>, parent: *mut MPI_Comm) -> c_int
    where
        F: FnOnce(*mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(parent) }
    }
    #[inline]
    fn comm_join<F>(next_f: UnsafeBox<F>, fd: c_int, intercomm: *mut MPI_Comm) -> c_int
    where
        F: FnOnce(c_int, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(fd, intercomm) }
    }
    #[inline]
    fn lookup_name<F>(
        next_f: UnsafeBox<F>,
        service_name: *const c_char,
        info: MPI_Info,
        port_name: *mut c_char,
    ) -> c_int
    where
        F: FnOnce(*const c_char, MPI_Info, *mut c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(service_name, info, port_name) }
    }
    #[inline]
    fn open_port<F>(next_f: UnsafeBox<F>, info: MPI_Info, port_name: *mut c_char) -> c_int
    where
        F: FnOnce(MPI_Info, *mut c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(info, port_name) }
    }
    #[inline]
    fn publish_name<F>(
        next_f: UnsafeBox<F>,
        service_name: *const c_char,
        info: MPI_Info,
        port_name: *const c_char,
    ) -> c_int
    where
        F: FnOnce(*const c_char, MPI_Info, *const c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(service_name, info, port_name) }
    }
    #[inline]
    fn unpublish_name<F>(
        next_f: UnsafeBox<F>,
        service_name: *const c_char,
        info: MPI_Info,
        port_name: *const c_char,
    ) -> c_int
    where
        F: FnOnce(*const c_char, MPI_Info, *const c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(service_name, info, port_name) }
    }
    #[inline]
    fn comm_set_info<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, info: MPI_Info) -> c_int
    where
        F: FnOnce(MPI_Comm, MPI_Info) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, info) }
    }
    #[inline]
    fn comm_get_info<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, info: *mut MPI_Info) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut MPI_Info) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, info) }
    }
    #[inline]
    fn accumulate<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: MPI_Datatype,
        target_rank: c_int,
        target_disp: MPI_Aint,
        target_count: c_int,
        target_datatype: MPI_Datatype,
        op: MPI_Op,
        win: MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Aint,
            c_int,
            MPI_Datatype,
            MPI_Op,
            MPI_Win,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                origin_addr,
                origin_count,
                origin_datatype,
                target_rank,
                target_disp,
                target_count,
                target_datatype,
                op,
                win,
            )
        }
    }
    #[inline]
    fn get<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *mut c_void,
        origin_count: c_int,
        origin_datatype: MPI_Datatype,
        target_rank: c_int,
        target_disp: MPI_Aint,
        target_count: c_int,
        target_datatype: MPI_Datatype,
        win: MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Aint,
            c_int,
            MPI_Datatype,
            MPI_Win,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                origin_addr,
                origin_count,
                origin_datatype,
                target_rank,
                target_disp,
                target_count,
                target_datatype,
                win,
            )
        }
    }
    #[inline]
    fn put<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: MPI_Datatype,
        target_rank: c_int,
        target_disp: MPI_Aint,
        target_count: c_int,
        target_datatype: MPI_Datatype,
        win: MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Aint,
            c_int,
            MPI_Datatype,
            MPI_Win,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                origin_addr,
                origin_count,
                origin_datatype,
                target_rank,
                target_disp,
                target_count,
                target_datatype,
                win,
            )
        }
    }
    #[inline]
    fn win_complete<F>(next_f: UnsafeBox<F>, win: MPI_Win) -> c_int
    where
        F: FnOnce(MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_create<F>(
        next_f: UnsafeBox<F>,
        base: *mut c_void,
        size: MPI_Aint,
        disp_unit: c_int,
        info: MPI_Info,
        comm: MPI_Comm,
        win: *mut MPI_Win,
    ) -> c_int
    where
        F: FnOnce(*mut c_void, MPI_Aint, c_int, MPI_Info, MPI_Comm, *mut MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(base, size, disp_unit, info, comm, win) }
    }
    #[inline]
    fn win_fence<F>(next_f: UnsafeBox<F>, assert: c_int, win: MPI_Win) -> c_int
    where
        F: FnOnce(c_int, MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(assert, win) }
    }
    #[inline]
    fn win_free<F>(next_f: UnsafeBox<F>, win: *mut MPI_Win) -> c_int
    where
        F: FnOnce(*mut MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_get_group<F>(next_f: UnsafeBox<F>, win: MPI_Win, group: *mut MPI_Group) -> c_int
    where
        F: FnOnce(MPI_Win, *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(win, group) }
    }
    #[inline]
    fn win_lock<F>(
        next_f: UnsafeBox<F>,
        lock_type: c_int,
        rank: c_int,
        assert: c_int,
        win: MPI_Win,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, c_int, MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(lock_type, rank, assert, win) }
    }
    #[inline]
    fn win_post<F>(next_f: UnsafeBox<F>, group: MPI_Group, assert: c_int, win: MPI_Win) -> c_int
    where
        F: FnOnce(MPI_Group, c_int, MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(group, assert, win) }
    }
    #[inline]
    fn win_start<F>(next_f: UnsafeBox<F>, group: MPI_Group, assert: c_int, win: MPI_Win) -> c_int
    where
        F: FnOnce(MPI_Group, c_int, MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(group, assert, win) }
    }
    #[inline]
    fn win_test<F>(next_f: UnsafeBox<F>, win: MPI_Win, flag: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Win, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(win, flag) }
    }
    #[inline]
    fn win_unlock<F>(next_f: UnsafeBox<F>, rank: c_int, win: MPI_Win) -> c_int
    where
        F: FnOnce(c_int, MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(rank, win) }
    }
    #[inline]
    fn win_wait<F>(next_f: UnsafeBox<F>, win: MPI_Win) -> c_int
    where
        F: FnOnce(MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_allocate<F>(
        next_f: UnsafeBox<F>,
        size: MPI_Aint,
        disp_unit: c_int,
        info: MPI_Info,
        comm: MPI_Comm,
        baseptr: *mut c_void,
        win: *mut MPI_Win,
    ) -> c_int
    where
        F: FnOnce(MPI_Aint, c_int, MPI_Info, MPI_Comm, *mut c_void, *mut MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(size, disp_unit, info, comm, baseptr, win) }
    }
    #[inline]
    fn win_allocate_shared<F>(
        next_f: UnsafeBox<F>,
        size: MPI_Aint,
        disp_unit: c_int,
        info: MPI_Info,
        comm: MPI_Comm,
        baseptr: *mut c_void,
        win: *mut MPI_Win,
    ) -> c_int
    where
        F: FnOnce(MPI_Aint, c_int, MPI_Info, MPI_Comm, *mut c_void, *mut MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(size, disp_unit, info, comm, baseptr, win) }
    }
    #[inline]
    fn win_shared_query<F>(
        next_f: UnsafeBox<F>,
        win: MPI_Win,
        rank: c_int,
        size: *mut MPI_Aint,
        disp_unit: *mut c_int,
        baseptr: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(MPI_Win, c_int, *mut MPI_Aint, *mut c_int, *mut c_void) -> c_int,
    {
        unsafe { next_f.unwrap()(win, rank, size, disp_unit, baseptr) }
    }
    #[inline]
    fn win_create_dynamic<F>(
        next_f: UnsafeBox<F>,
        info: MPI_Info,
        comm: MPI_Comm,
        win: *mut MPI_Win,
    ) -> c_int
    where
        F: FnOnce(MPI_Info, MPI_Comm, *mut MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(info, comm, win) }
    }
    #[inline]
    fn win_attach<F>(next_f: UnsafeBox<F>, win: MPI_Win, base: *mut c_void, size: MPI_Aint) -> c_int
    where
        F: FnOnce(MPI_Win, *mut c_void, MPI_Aint) -> c_int,
    {
        unsafe { next_f.unwrap()(win, base, size) }
    }
    #[inline]
    fn win_detach<F>(next_f: UnsafeBox<F>, win: MPI_Win, base: *const c_void) -> c_int
    where
        F: FnOnce(MPI_Win, *const c_void) -> c_int,
    {
        unsafe { next_f.unwrap()(win, base) }
    }
    #[inline]
    fn win_get_info<F>(next_f: UnsafeBox<F>, win: MPI_Win, info_used: *mut MPI_Info) -> c_int
    where
        F: FnOnce(MPI_Win, *mut MPI_Info) -> c_int,
    {
        unsafe { next_f.unwrap()(win, info_used) }
    }
    #[inline]
    fn win_set_info<F>(next_f: UnsafeBox<F>, win: MPI_Win, info: MPI_Info) -> c_int
    where
        F: FnOnce(MPI_Win, MPI_Info) -> c_int,
    {
        unsafe { next_f.unwrap()(win, info) }
    }
    #[inline]
    fn get_accumulate<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: MPI_Datatype,
        result_addr: *mut c_void,
        result_count: c_int,
        result_datatype: MPI_Datatype,
        target_rank: c_int,
        target_disp: MPI_Aint,
        target_count: c_int,
        target_datatype: MPI_Datatype,
        op: MPI_Op,
        win: MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Aint,
            c_int,
            MPI_Datatype,
            MPI_Op,
            MPI_Win,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                origin_addr,
                origin_count,
                origin_datatype,
                result_addr,
                result_count,
                result_datatype,
                target_rank,
                target_disp,
                target_count,
                target_datatype,
                op,
                win,
            )
        }
    }
    #[inline]
    fn fetch_and_op<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *const c_void,
        result_addr: *mut c_void,
        datatype: MPI_Datatype,
        target_rank: c_int,
        target_disp: MPI_Aint,
        op: MPI_Op,
        win: MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            MPI_Datatype,
            c_int,
            MPI_Aint,
            MPI_Op,
            MPI_Win,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                origin_addr,
                result_addr,
                datatype,
                target_rank,
                target_disp,
                op,
                win,
            )
        }
    }
    #[inline]
    fn compare_and_swap<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *const c_void,
        compare_addr: *const c_void,
        result_addr: *mut c_void,
        datatype: MPI_Datatype,
        target_rank: c_int,
        target_disp: MPI_Aint,
        win: MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_void,
            *mut c_void,
            MPI_Datatype,
            c_int,
            MPI_Aint,
            MPI_Win,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                origin_addr,
                compare_addr,
                result_addr,
                datatype,
                target_rank,
                target_disp,
                win,
            )
        }
    }
    #[inline]
    fn rput<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: MPI_Datatype,
        target_rank: c_int,
        target_disp: MPI_Aint,
        target_count: c_int,
        target_datatype: MPI_Datatype,
        win: MPI_Win,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Aint,
            c_int,
            MPI_Datatype,
            MPI_Win,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                origin_addr,
                origin_count,
                origin_datatype,
                target_rank,
                target_disp,
                target_count,
                target_datatype,
                win,
                request,
            )
        }
    }
    #[inline]
    fn rget<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *mut c_void,
        origin_count: c_int,
        origin_datatype: MPI_Datatype,
        target_rank: c_int,
        target_disp: MPI_Aint,
        target_count: c_int,
        target_datatype: MPI_Datatype,
        win: MPI_Win,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Aint,
            c_int,
            MPI_Datatype,
            MPI_Win,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                origin_addr,
                origin_count,
                origin_datatype,
                target_rank,
                target_disp,
                target_count,
                target_datatype,
                win,
                request,
            )
        }
    }
    #[inline]
    fn raccumulate<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: MPI_Datatype,
        target_rank: c_int,
        target_disp: MPI_Aint,
        target_count: c_int,
        target_datatype: MPI_Datatype,
        op: MPI_Op,
        win: MPI_Win,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Aint,
            c_int,
            MPI_Datatype,
            MPI_Op,
            MPI_Win,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                origin_addr,
                origin_count,
                origin_datatype,
                target_rank,
                target_disp,
                target_count,
                target_datatype,
                op,
                win,
                request,
            )
        }
    }
    #[inline]
    fn rget_accumulate<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: MPI_Datatype,
        result_addr: *mut c_void,
        result_count: c_int,
        result_datatype: MPI_Datatype,
        target_rank: c_int,
        target_disp: MPI_Aint,
        target_count: c_int,
        target_datatype: MPI_Datatype,
        op: MPI_Op,
        win: MPI_Win,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Aint,
            c_int,
            MPI_Datatype,
            MPI_Op,
            MPI_Win,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                origin_addr,
                origin_count,
                origin_datatype,
                result_addr,
                result_count,
                result_datatype,
                target_rank,
                target_disp,
                target_count,
                target_datatype,
                op,
                win,
                request,
            )
        }
    }
    #[inline]
    fn win_lock_all<F>(next_f: UnsafeBox<F>, assert: c_int, win: MPI_Win) -> c_int
    where
        F: FnOnce(c_int, MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(assert, win) }
    }
    #[inline]
    fn win_unlock_all<F>(next_f: UnsafeBox<F>, win: MPI_Win) -> c_int
    where
        F: FnOnce(MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_flush<F>(next_f: UnsafeBox<F>, rank: c_int, win: MPI_Win) -> c_int
    where
        F: FnOnce(c_int, MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(rank, win) }
    }
    #[inline]
    fn win_flush_all<F>(next_f: UnsafeBox<F>, win: MPI_Win) -> c_int
    where
        F: FnOnce(MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_flush_local<F>(next_f: UnsafeBox<F>, rank: c_int, win: MPI_Win) -> c_int
    where
        F: FnOnce(c_int, MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(rank, win) }
    }
    #[inline]
    fn win_flush_local_all<F>(next_f: UnsafeBox<F>, win: MPI_Win) -> c_int
    where
        F: FnOnce(MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_sync<F>(next_f: UnsafeBox<F>, win: MPI_Win) -> c_int
    where
        F: FnOnce(MPI_Win) -> c_int,
    {
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn add_error_class<F>(next_f: UnsafeBox<F>, errorclass: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(errorclass) }
    }
    #[inline]
    fn add_error_code<F>(next_f: UnsafeBox<F>, errorclass: c_int, errorcode: *mut c_int) -> c_int
    where
        F: FnOnce(c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(errorclass, errorcode) }
    }
    #[inline]
    fn add_error_string<F>(next_f: UnsafeBox<F>, errorcode: c_int, string: *const c_char) -> c_int
    where
        F: FnOnce(c_int, *const c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(errorcode, string) }
    }
    #[inline]
    fn comm_call_errhandler<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, errorcode: c_int) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, errorcode) }
    }
    #[inline]
    fn comm_create_keyval<F>(
        next_f: UnsafeBox<F>,
        comm_copy_attr_fn: MPI_Comm_copy_attr_function,
        comm_delete_attr_fn: MPI_Comm_delete_attr_function,
        comm_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(
            MPI_Comm_copy_attr_function,
            MPI_Comm_delete_attr_function,
            *mut c_int,
            *mut c_void,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                comm_copy_attr_fn,
                comm_delete_attr_fn,
                comm_keyval,
                extra_state,
            )
        }
    }
    #[inline]
    fn comm_delete_attr<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, comm_keyval: c_int) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, comm_keyval) }
    }
    #[inline]
    fn comm_free_keyval<F>(next_f: UnsafeBox<F>, comm_keyval: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm_keyval) }
    }
    #[inline]
    fn comm_get_attr<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *mut c_void, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, comm_keyval, attribute_val, flag) }
    }
    #[inline]
    fn comm_get_name<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        comm_name: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut c_char, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, comm_name, resultlen) }
    }
    #[inline]
    fn comm_set_attr<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *mut c_void) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, comm_keyval, attribute_val) }
    }
    #[inline]
    fn comm_set_name<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, comm_name: *const c_char) -> c_int
    where
        F: FnOnce(MPI_Comm, *const c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, comm_name) }
    }
    #[inline]
    fn file_call_errhandler<F>(next_f: UnsafeBox<F>, fh: MPI_File, errorcode: c_int) -> c_int
    where
        F: FnOnce(MPI_File, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, errorcode) }
    }
    #[inline]
    fn grequest_complete<F>(next_f: UnsafeBox<F>, request: MPI_Request) -> c_int
    where
        F: FnOnce(MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(request) }
    }
    #[inline]
    fn grequest_start<F>(
        next_f: UnsafeBox<F>,
        query_fn: MPI_Grequest_query_function,
        free_fn: MPI_Grequest_free_function,
        cancel_fn: MPI_Grequest_cancel_function,
        extra_state: *mut c_void,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            MPI_Grequest_query_function,
            MPI_Grequest_free_function,
            MPI_Grequest_cancel_function,
            *mut c_void,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(query_fn, free_fn, cancel_fn, extra_state, request) }
    }
    #[inline]
    fn init_thread<F>(
        next_f: UnsafeBox<F>,
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        required: c_int,
        provided: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(*mut c_int, *mut *mut *mut c_char, c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(argc, argv, required, provided) }
    }
    #[inline]
    fn is_thread_main<F>(next_f: UnsafeBox<F>, flag: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(flag) }
    }
    #[inline]
    fn query_thread<F>(next_f: UnsafeBox<F>, provided: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(provided) }
    }
    #[inline]
    fn status_set_cancelled<F>(next_f: UnsafeBox<F>, status: *mut MPI_Status, flag: c_int) -> c_int
    where
        F: FnOnce(*mut MPI_Status, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(status, flag) }
    }
    #[inline]
    fn status_set_elements<F>(
        next_f: UnsafeBox<F>,
        status: *mut MPI_Status,
        datatype: MPI_Datatype,
        count: c_int,
    ) -> c_int
    where
        F: FnOnce(*mut MPI_Status, MPI_Datatype, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(status, datatype, count) }
    }
    #[inline]
    fn type_create_keyval<F>(
        next_f: UnsafeBox<F>,
        type_copy_attr_fn: MPI_Type_copy_attr_function,
        type_delete_attr_fn: MPI_Type_delete_attr_function,
        type_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(
            MPI_Type_copy_attr_function,
            MPI_Type_delete_attr_function,
            *mut c_int,
            *mut c_void,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                type_copy_attr_fn,
                type_delete_attr_fn,
                type_keyval,
                extra_state,
            )
        }
    }
    #[inline]
    fn type_delete_attr<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        type_keyval: c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, type_keyval) }
    }
    #[inline]
    fn type_dup<F>(next_f: UnsafeBox<F>, oldtype: MPI_Datatype, newtype: *mut MPI_Datatype) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(oldtype, newtype) }
    }
    #[inline]
    fn type_free_keyval<F>(next_f: UnsafeBox<F>, type_keyval: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(type_keyval) }
    }
    #[inline]
    fn type_get_attr<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        type_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, c_int, *mut c_void, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, type_keyval, attribute_val, flag) }
    }
    #[inline]
    fn type_get_contents<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        max_integers: c_int,
        max_addresses: c_int,
        max_datatypes: c_int,
        array_of_integers: *mut c_int,
        array_of_addresses: *mut MPI_Aint,
        array_of_datatypes: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            MPI_Datatype,
            c_int,
            c_int,
            c_int,
            *mut c_int,
            *mut MPI_Aint,
            *mut MPI_Datatype,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                datatype,
                max_integers,
                max_addresses,
                max_datatypes,
                array_of_integers,
                array_of_addresses,
                array_of_datatypes,
            )
        }
    }
    #[inline]
    fn type_get_envelope<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        num_integers: *mut c_int,
        num_addresses: *mut c_int,
        num_datatypes: *mut c_int,
        combiner: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut c_int, *mut c_int, *mut c_int, *mut c_int) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                datatype,
                num_integers,
                num_addresses,
                num_datatypes,
                combiner,
            )
        }
    }
    #[inline]
    fn type_get_name<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        type_name: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut c_char, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, type_name, resultlen) }
    }
    #[inline]
    fn type_set_attr<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        type_keyval: c_int,
        attribute_val: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, c_int, *mut c_void) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, type_keyval, attribute_val) }
    }
    #[inline]
    fn type_set_name<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        type_name: *const c_char,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, *const c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, type_name) }
    }
    #[inline]
    fn type_match_size<F>(
        next_f: UnsafeBox<F>,
        typeclass: c_int,
        size: c_int,
        datatype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(typeclass, size, datatype) }
    }
    #[inline]
    fn win_call_errhandler<F>(next_f: UnsafeBox<F>, win: MPI_Win, errorcode: c_int) -> c_int
    where
        F: FnOnce(MPI_Win, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(win, errorcode) }
    }
    #[inline]
    fn win_create_keyval<F>(
        next_f: UnsafeBox<F>,
        win_copy_attr_fn: MPI_Win_copy_attr_function,
        win_delete_attr_fn: MPI_Win_delete_attr_function,
        win_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(
            MPI_Win_copy_attr_function,
            MPI_Win_delete_attr_function,
            *mut c_int,
            *mut c_void,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                win_copy_attr_fn,
                win_delete_attr_fn,
                win_keyval,
                extra_state,
            )
        }
    }
    #[inline]
    fn win_delete_attr<F>(next_f: UnsafeBox<F>, win: MPI_Win, win_keyval: c_int) -> c_int
    where
        F: FnOnce(MPI_Win, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(win, win_keyval) }
    }
    #[inline]
    fn win_free_keyval<F>(next_f: UnsafeBox<F>, win_keyval: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(win_keyval) }
    }
    #[inline]
    fn win_get_attr<F>(
        next_f: UnsafeBox<F>,
        win: MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Win, c_int, *mut c_void, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(win, win_keyval, attribute_val, flag) }
    }
    #[inline]
    fn win_get_name<F>(
        next_f: UnsafeBox<F>,
        win: MPI_Win,
        win_name: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Win, *mut c_char, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(win, win_name, resultlen) }
    }
    #[inline]
    fn win_set_attr<F>(
        next_f: UnsafeBox<F>,
        win: MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(MPI_Win, c_int, *mut c_void) -> c_int,
    {
        unsafe { next_f.unwrap()(win, win_keyval, attribute_val) }
    }
    #[inline]
    fn win_set_name<F>(next_f: UnsafeBox<F>, win: MPI_Win, win_name: *const c_char) -> c_int
    where
        F: FnOnce(MPI_Win, *const c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(win, win_name) }
    }
    #[inline]
    fn alloc_mem<F>(
        next_f: UnsafeBox<F>,
        size: MPI_Aint,
        info: MPI_Info,
        baseptr: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(MPI_Aint, MPI_Info, *mut c_void) -> c_int,
    {
        unsafe { next_f.unwrap()(size, info, baseptr) }
    }
    #[inline]
    fn comm_create_errhandler<F>(
        next_f: UnsafeBox<F>,
        comm_errhandler_fn: MPI_Comm_errhandler_function,
        errhandler: *mut MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm_errhandler_function, *mut MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(comm_errhandler_fn, errhandler) }
    }
    #[inline]
    fn comm_get_errhandler<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        errhandler: *mut MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, errhandler) }
    }
    #[inline]
    fn comm_set_errhandler<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        errhandler: MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, errhandler) }
    }
    #[inline]
    fn file_create_errhandler<F>(
        next_f: UnsafeBox<F>,
        file_errhandler_fn: MPI_File_errhandler_function,
        errhandler: *mut MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_File_errhandler_function, *mut MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(file_errhandler_fn, errhandler) }
    }
    #[inline]
    fn file_get_errhandler<F>(
        next_f: UnsafeBox<F>,
        file: MPI_File,
        errhandler: *mut MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(file, errhandler) }
    }
    #[inline]
    fn file_set_errhandler<F>(
        next_f: UnsafeBox<F>,
        file: MPI_File,
        errhandler: MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(file, errhandler) }
    }
    #[inline]
    fn finalized<F>(next_f: UnsafeBox<F>, flag: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(flag) }
    }
    #[inline]
    fn free_mem<F>(next_f: UnsafeBox<F>, base: *mut c_void) -> c_int
    where
        F: FnOnce(*mut c_void) -> c_int,
    {
        unsafe { next_f.unwrap()(base) }
    }
    #[inline]
    fn get_address<F>(
        next_f: UnsafeBox<F>,
        location: *const c_void,
        address: *mut MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(*const c_void, *mut MPI_Aint) -> c_int,
    {
        unsafe { next_f.unwrap()(location, address) }
    }
    #[inline]
    fn info_create<F>(next_f: UnsafeBox<F>, info: *mut MPI_Info) -> c_int
    where
        F: FnOnce(*mut MPI_Info) -> c_int,
    {
        unsafe { next_f.unwrap()(info) }
    }
    #[inline]
    fn info_delete<F>(next_f: UnsafeBox<F>, info: MPI_Info, key: *const c_char) -> c_int
    where
        F: FnOnce(MPI_Info, *const c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(info, key) }
    }
    #[inline]
    fn info_dup<F>(next_f: UnsafeBox<F>, info: MPI_Info, newinfo: *mut MPI_Info) -> c_int
    where
        F: FnOnce(MPI_Info, *mut MPI_Info) -> c_int,
    {
        unsafe { next_f.unwrap()(info, newinfo) }
    }
    #[inline]
    fn info_free<F>(next_f: UnsafeBox<F>, info: *mut MPI_Info) -> c_int
    where
        F: FnOnce(*mut MPI_Info) -> c_int,
    {
        unsafe { next_f.unwrap()(info) }
    }
    #[inline]
    fn info_get<F>(
        next_f: UnsafeBox<F>,
        info: MPI_Info,
        key: *const c_char,
        valuelen: c_int,
        value: *mut c_char,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Info, *const c_char, c_int, *mut c_char, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(info, key, valuelen, value, flag) }
    }
    #[inline]
    fn info_get_nkeys<F>(next_f: UnsafeBox<F>, info: MPI_Info, nkeys: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Info, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(info, nkeys) }
    }
    #[inline]
    fn info_get_nthkey<F>(next_f: UnsafeBox<F>, info: MPI_Info, n: c_int, key: *mut c_char) -> c_int
    where
        F: FnOnce(MPI_Info, c_int, *mut c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(info, n, key) }
    }
    #[inline]
    fn info_get_valuelen<F>(
        next_f: UnsafeBox<F>,
        info: MPI_Info,
        key: *const c_char,
        valuelen: *mut c_int,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Info, *const c_char, *mut c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(info, key, valuelen, flag) }
    }
    #[inline]
    fn info_set<F>(
        next_f: UnsafeBox<F>,
        info: MPI_Info,
        key: *const c_char,
        value: *const c_char,
    ) -> c_int
    where
        F: FnOnce(MPI_Info, *const c_char, *const c_char) -> c_int,
    {
        unsafe { next_f.unwrap()(info, key, value) }
    }
    #[inline]
    fn pack_external<F>(
        next_f: UnsafeBox<F>,
        datarep: *const c_char,
        inbuf: *const c_void,
        incount: c_int,
        datatype: MPI_Datatype,
        outbuf: *mut c_void,
        outsize: MPI_Aint,
        position: *mut MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(
            *const c_char,
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            MPI_Aint,
            *mut MPI_Aint,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(datarep, inbuf, incount, datatype, outbuf, outsize, position) }
    }
    #[inline]
    fn pack_external_size<F>(
        next_f: UnsafeBox<F>,
        datarep: *const c_char,
        incount: c_int,
        datatype: MPI_Datatype,
        size: *mut MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(*const c_char, c_int, MPI_Datatype, *mut MPI_Aint) -> c_int,
    {
        unsafe { next_f.unwrap()(datarep, incount, datatype, size) }
    }
    #[inline]
    fn request_get_status<F>(
        next_f: UnsafeBox<F>,
        request: MPI_Request,
        flag: *mut c_int,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_Request, *mut c_int, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(request, flag, status) }
    }
    #[inline]
    fn type_create_darray<F>(
        next_f: UnsafeBox<F>,
        size: c_int,
        rank: c_int,
        ndims: c_int,
        array_of_gsizes: *const c_int,
        array_of_distribs: *const c_int,
        array_of_dargs: *const c_int,
        array_of_psizes: *const c_int,
        order: c_int,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            c_int,
            c_int,
            *const c_int,
            *const c_int,
            *const c_int,
            *const c_int,
            c_int,
            MPI_Datatype,
            *mut MPI_Datatype,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                size,
                rank,
                ndims,
                array_of_gsizes,
                array_of_distribs,
                array_of_dargs,
                array_of_psizes,
                order,
                oldtype,
                newtype,
            )
        }
    }
    #[inline]
    fn type_create_hindexed<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const MPI_Aint,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, *const c_int, *const MPI_Aint, MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                count,
                array_of_blocklengths,
                array_of_displacements,
                oldtype,
                newtype,
            )
        }
    }
    #[inline]
    fn type_create_hvector<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        blocklength: c_int,
        stride: MPI_Aint,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, MPI_Aint, MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(count, blocklength, stride, oldtype, newtype) }
    }
    #[inline]
    fn type_create_indexed_block<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        blocklength: c_int,
        array_of_displacements: *const c_int,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, *const c_int, MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(count, blocklength, array_of_displacements, oldtype, newtype) }
    }
    #[inline]
    fn type_create_hindexed_block<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        blocklength: c_int,
        array_of_displacements: *const MPI_Aint,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, *const MPI_Aint, MPI_Datatype, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(count, blocklength, array_of_displacements, oldtype, newtype) }
    }
    #[inline]
    fn type_create_resized<F>(
        next_f: UnsafeBox<F>,
        oldtype: MPI_Datatype,
        lb: MPI_Aint,
        extent: MPI_Aint,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, MPI_Aint, MPI_Aint, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(oldtype, lb, extent, newtype) }
    }
    #[inline]
    fn type_create_struct<F>(
        next_f: UnsafeBox<F>,
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const MPI_Aint,
        array_of_types: *const MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *const c_int,
            *const MPI_Aint,
            *const MPI_Datatype,
            *mut MPI_Datatype,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                count,
                array_of_blocklengths,
                array_of_displacements,
                array_of_types,
                newtype,
            )
        }
    }
    #[inline]
    fn type_create_subarray<F>(
        next_f: UnsafeBox<F>,
        ndims: c_int,
        array_of_sizes: *const c_int,
        array_of_subsizes: *const c_int,
        array_of_starts: *const c_int,
        order: c_int,
        oldtype: MPI_Datatype,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *const c_int,
            *const c_int,
            *const c_int,
            c_int,
            MPI_Datatype,
            *mut MPI_Datatype,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                ndims,
                array_of_sizes,
                array_of_subsizes,
                array_of_starts,
                order,
                oldtype,
                newtype,
            )
        }
    }
    #[inline]
    fn type_get_extent<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        lb: *mut MPI_Aint,
        extent: *mut MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut MPI_Aint, *mut MPI_Aint) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, lb, extent) }
    }
    #[inline]
    fn type_get_true_extent<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        true_lb: *mut MPI_Aint,
        true_extent: *mut MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut MPI_Aint, *mut MPI_Aint) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, true_lb, true_extent) }
    }
    #[inline]
    fn unpack_external<F>(
        next_f: UnsafeBox<F>,
        datarep: *const c_char,
        inbuf: *const c_void,
        insize: MPI_Aint,
        position: *mut MPI_Aint,
        outbuf: *mut c_void,
        outcount: c_int,
        datatype: MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            *const c_char,
            *const c_void,
            MPI_Aint,
            *mut MPI_Aint,
            *mut c_void,
            c_int,
            MPI_Datatype,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(datarep, inbuf, insize, position, outbuf, outcount, datatype) }
    }
    #[inline]
    fn win_create_errhandler<F>(
        next_f: UnsafeBox<F>,
        win_errhandler_fn: MPI_Win_errhandler_function,
        errhandler: *mut MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_Win_errhandler_function, *mut MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(win_errhandler_fn, errhandler) }
    }
    #[inline]
    fn win_get_errhandler<F>(
        next_f: UnsafeBox<F>,
        win: MPI_Win,
        errhandler: *mut MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_Win, *mut MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(win, errhandler) }
    }
    #[inline]
    fn win_set_errhandler<F>(
        next_f: UnsafeBox<F>,
        win: MPI_Win,
        errhandler: MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(MPI_Win, MPI_Errhandler) -> c_int,
    {
        unsafe { next_f.unwrap()(win, errhandler) }
    }
    #[inline]
    fn type_create_f90_integer<F>(
        next_f: UnsafeBox<F>,
        range: c_int,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(range, newtype) }
    }
    #[inline]
    fn type_create_f90_real<F>(
        next_f: UnsafeBox<F>,
        precision: c_int,
        range: c_int,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(precision, range, newtype) }
    }
    #[inline]
    fn type_create_f90_complex<F>(
        next_f: UnsafeBox<F>,
        precision: c_int,
        range: c_int,
        newtype: *mut MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, *mut MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(precision, range, newtype) }
    }
    #[inline]
    fn reduce_local<F>(
        next_f: UnsafeBox<F>,
        inbuf: *const c_void,
        inoutbuf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
    ) -> c_int
    where
        F: FnOnce(*const c_void, *mut c_void, c_int, MPI_Datatype, MPI_Op) -> c_int,
    {
        unsafe { next_f.unwrap()(inbuf, inoutbuf, count, datatype, op) }
    }
    #[inline]
    fn op_commutative<F>(next_f: UnsafeBox<F>, op: MPI_Op, commute: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_Op, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(op, commute) }
    }
    #[inline]
    fn reduce_scatter_block<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcount: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_void, *mut c_void, c_int, MPI_Datatype, MPI_Op, MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, recvcount, datatype, op, comm) }
    }
    #[inline]
    fn dist_graph_create_adjacent<F>(
        next_f: UnsafeBox<F>,
        comm_old: MPI_Comm,
        indegree: c_int,
        sources: *const c_int,
        sourceweights: *const c_int,
        outdegree: c_int,
        destinations: *const c_int,
        destweights: *const c_int,
        info: MPI_Info,
        reorder: c_int,
        comm_dist_graph: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            MPI_Comm,
            c_int,
            *const c_int,
            *const c_int,
            c_int,
            *const c_int,
            *const c_int,
            MPI_Info,
            c_int,
            *mut MPI_Comm,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                comm_old,
                indegree,
                sources,
                sourceweights,
                outdegree,
                destinations,
                destweights,
                info,
                reorder,
                comm_dist_graph,
            )
        }
    }
    #[inline]
    fn dist_graph_create<F>(
        next_f: UnsafeBox<F>,
        comm_old: MPI_Comm,
        n: c_int,
        sources: *const c_int,
        degrees: *const c_int,
        destinations: *const c_int,
        weights: *const c_int,
        info: MPI_Info,
        reorder: c_int,
        comm_dist_graph: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            MPI_Comm,
            c_int,
            *const c_int,
            *const c_int,
            *const c_int,
            *const c_int,
            MPI_Info,
            c_int,
            *mut MPI_Comm,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                comm_old,
                n,
                sources,
                degrees,
                destinations,
                weights,
                info,
                reorder,
                comm_dist_graph,
            )
        }
    }
    #[inline]
    fn dist_graph_neighbors_count<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        indegree: *mut c_int,
        outdegree: *mut c_int,
        weighted: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut c_int, *mut c_int, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, indegree, outdegree, weighted) }
    }
    #[inline]
    fn dist_graph_neighbors<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        maxindegree: c_int,
        sources: *mut c_int,
        sourceweights: *mut c_int,
        maxoutdegree: c_int,
        destinations: *mut c_int,
        destweights: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, *mut c_int, *mut c_int, c_int, *mut c_int, *mut c_int) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                comm,
                maxindegree,
                sources,
                sourceweights,
                maxoutdegree,
                destinations,
                destweights,
            )
        }
    }
    #[inline]
    fn improbe<F>(
        next_f: UnsafeBox<F>,
        source: c_int,
        tag: c_int,
        comm: MPI_Comm,
        flag: *mut c_int,
        message: *mut MPI_Message,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, MPI_Comm, *mut c_int, *mut MPI_Message, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(source, tag, comm, flag, message, status) }
    }
    #[inline]
    fn imrecv<F>(
        next_f: UnsafeBox<F>,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        message: *mut MPI_Message,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(*mut c_void, c_int, MPI_Datatype, *mut MPI_Message, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(buf, count, datatype, message, request) }
    }
    #[inline]
    fn mprobe<F>(
        next_f: UnsafeBox<F>,
        source: c_int,
        tag: c_int,
        comm: MPI_Comm,
        message: *mut MPI_Message,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, MPI_Comm, *mut MPI_Message, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(source, tag, comm, message, status) }
    }
    #[inline]
    fn mrecv<F>(
        next_f: UnsafeBox<F>,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        message: *mut MPI_Message,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(*mut c_void, c_int, MPI_Datatype, *mut MPI_Message, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(buf, count, datatype, message, status) }
    }
    #[inline]
    fn comm_idup<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        newcomm: *mut MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut MPI_Comm, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, newcomm, request) }
    }
    #[inline]
    fn ibarrier<F>(next_f: UnsafeBox<F>, comm: MPI_Comm, request: *mut MPI_Request) -> c_int
    where
        F: FnOnce(MPI_Comm, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, request) }
    }
    #[inline]
    fn ibcast<F>(
        next_f: UnsafeBox<F>,
        buffer: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        root: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(*mut c_void, c_int, MPI_Datatype, c_int, MPI_Comm, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(buffer, count, datatype, root, comm, request) }
    }
    #[inline]
    fn igather<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        root: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm, request,
            )
        }
    }
    #[inline]
    fn igatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: MPI_Datatype,
        root: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, root, comm,
                request,
            )
        }
    }
    #[inline]
    fn iscatter<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        root: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm, request,
            )
        }
    }
    #[inline]
    fn iscatterv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        displs: *const c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        root: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            MPI_Datatype,
            c_int,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, displs, sendtype, recvbuf, recvcount, recvtype, root, comm,
                request,
            )
        }
    }
    #[inline]
    fn iallgather<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn iallgatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            MPI_Datatype,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn ialltoall<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn ialltoallv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, sdispls, sendtype, recvbuf, recvcounts, rdispls, recvtype,
                comm, request,
            )
        }
    }
    #[inline]
    fn ialltoallw<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtypes: *const MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtypes: *const MPI_Datatype,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            *const MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            *const MPI_Datatype,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, sdispls, sendtypes, recvbuf, recvcounts, rdispls, recvtypes,
                comm, request,
            )
        }
    }
    #[inline]
    fn ireduce<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        root: c_int,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Op,
            c_int,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, root, comm, request) }
    }
    #[inline]
    fn iallreduce<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Op,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm, request) }
    }
    #[inline]
    fn ireduce_scatter<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            *const c_int,
            MPI_Datatype,
            MPI_Op,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, recvcounts, datatype, op, comm, request) }
    }
    #[inline]
    fn ireduce_scatter_block<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcount: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Op,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, recvcount, datatype, op, comm, request) }
    }
    #[inline]
    fn iscan<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Op,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm, request) }
    }
    #[inline]
    fn iexscan<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        op: MPI_Op,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Op,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm, request) }
    }
    #[inline]
    fn ineighbor_allgather<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn ineighbor_allgatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            MPI_Datatype,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn ineighbor_alltoall<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut c_void,
            c_int,
            MPI_Datatype,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn ineighbor_alltoallv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
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
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, sdispls, sendtype, recvbuf, recvcounts, rdispls, recvtype,
                comm, request,
            )
        }
    }
    #[inline]
    fn ineighbor_alltoallw<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const MPI_Aint,
        sendtypes: *const MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const MPI_Aint,
        recvtypes: *const MPI_Datatype,
        comm: MPI_Comm,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const MPI_Aint,
            *const MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const MPI_Aint,
            *const MPI_Datatype,
            MPI_Comm,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, sdispls, sendtypes, recvbuf, recvcounts, rdispls, recvtypes,
                comm, request,
            )
        }
    }
    #[inline]
    fn neighbor_allgather<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
            )
        }
    }
    #[inline]
    fn neighbor_allgatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm,
            )
        }
    }
    #[inline]
    fn neighbor_alltoall<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
            )
        }
    }
    #[inline]
    fn neighbor_alltoallv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
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
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, sdispls, sendtype, recvbuf, recvcounts, rdispls, recvtype,
                comm,
            )
        }
    }
    #[inline]
    fn neighbor_alltoallw<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const MPI_Aint,
        sendtypes: *const MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const MPI_Aint,
        recvtypes: *const MPI_Datatype,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const MPI_Aint,
            *const MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const MPI_Aint,
            *const MPI_Datatype,
            MPI_Comm,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, sdispls, sendtypes, recvbuf, recvcounts, rdispls, recvtypes,
                comm,
            )
        }
    }
    #[inline]
    fn comm_split_type<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        split_type: c_int,
        key: c_int,
        info: MPI_Info,
        newcomm: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, c_int, c_int, MPI_Info, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, split_type, key, info, newcomm) }
    }
    #[inline]
    fn get_elements_x<F>(
        next_f: UnsafeBox<F>,
        status: *const MPI_Status,
        datatype: MPI_Datatype,
        count: *mut MPI_Count,
    ) -> c_int
    where
        F: FnOnce(*const MPI_Status, MPI_Datatype, *mut MPI_Count) -> c_int,
    {
        unsafe { next_f.unwrap()(status, datatype, count) }
    }
    #[inline]
    fn status_set_elements_x<F>(
        next_f: UnsafeBox<F>,
        status: *mut MPI_Status,
        datatype: MPI_Datatype,
        count: MPI_Count,
    ) -> c_int
    where
        F: FnOnce(*mut MPI_Status, MPI_Datatype, MPI_Count) -> c_int,
    {
        unsafe { next_f.unwrap()(status, datatype, count) }
    }
    #[inline]
    fn type_get_extent_x<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        lb: *mut MPI_Count,
        extent: *mut MPI_Count,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut MPI_Count, *mut MPI_Count) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, lb, extent) }
    }
    #[inline]
    fn type_get_true_extent_x<F>(
        next_f: UnsafeBox<F>,
        datatype: MPI_Datatype,
        lb: *mut MPI_Count,
        extent: *mut MPI_Count,
    ) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut MPI_Count, *mut MPI_Count) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, lb, extent) }
    }
    #[inline]
    fn type_size_x<F>(next_f: UnsafeBox<F>, datatype: MPI_Datatype, size: *mut MPI_Count) -> c_int
    where
        F: FnOnce(MPI_Datatype, *mut MPI_Count) -> c_int,
    {
        unsafe { next_f.unwrap()(datatype, size) }
    }
    #[inline]
    fn comm_create_group<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        group: MPI_Group,
        tag: c_int,
        newcomm: *mut MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, MPI_Group, c_int, *mut MPI_Comm) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, group, tag, newcomm) }
    }
    #[inline]
    fn file_open<F>(
        next_f: UnsafeBox<F>,
        comm: MPI_Comm,
        filename: *const c_char,
        amode: c_int,
        info: MPI_Info,
        fh: *mut MPI_File,
    ) -> c_int
    where
        F: FnOnce(MPI_Comm, *const c_char, c_int, MPI_Info, *mut MPI_File) -> c_int,
    {
        unsafe { next_f.unwrap()(comm, filename, amode, info, fh) }
    }
    #[inline]
    fn file_close<F>(next_f: UnsafeBox<F>, fh: *mut MPI_File) -> c_int
    where
        F: FnOnce(*mut MPI_File) -> c_int,
    {
        unsafe { next_f.unwrap()(fh) }
    }
    #[inline]
    fn file_delete<F>(next_f: UnsafeBox<F>, filename: *const c_char, info: MPI_Info) -> c_int
    where
        F: FnOnce(*const c_char, MPI_Info) -> c_int,
    {
        unsafe { next_f.unwrap()(filename, info) }
    }
    #[inline]
    fn file_set_size<F>(next_f: UnsafeBox<F>, fh: MPI_File, size: MPI_Offset) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Offset) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, size) }
    }
    #[inline]
    fn file_preallocate<F>(next_f: UnsafeBox<F>, fh: MPI_File, size: MPI_Offset) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Offset) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, size) }
    }
    #[inline]
    fn file_get_size<F>(next_f: UnsafeBox<F>, fh: MPI_File, size: *mut MPI_Offset) -> c_int
    where
        F: FnOnce(MPI_File, *mut MPI_Offset) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, size) }
    }
    #[inline]
    fn file_get_group<F>(next_f: UnsafeBox<F>, fh: MPI_File, group: *mut MPI_Group) -> c_int
    where
        F: FnOnce(MPI_File, *mut MPI_Group) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, group) }
    }
    #[inline]
    fn file_get_amode<F>(next_f: UnsafeBox<F>, fh: MPI_File, amode: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, amode) }
    }
    #[inline]
    fn file_set_info<F>(next_f: UnsafeBox<F>, fh: MPI_File, info: MPI_Info) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Info) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, info) }
    }
    #[inline]
    fn file_get_info<F>(next_f: UnsafeBox<F>, fh: MPI_File, info_used: *mut MPI_Info) -> c_int
    where
        F: FnOnce(MPI_File, *mut MPI_Info) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, info_used) }
    }
    #[inline]
    fn file_set_view<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        disp: MPI_Offset,
        etype: MPI_Datatype,
        filetype: MPI_Datatype,
        datarep: *const c_char,
        info: MPI_Info,
    ) -> c_int
    where
        F: FnOnce(
            MPI_File,
            MPI_Offset,
            MPI_Datatype,
            MPI_Datatype,
            *const c_char,
            MPI_Info,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, disp, etype, filetype, datarep, info) }
    }
    #[inline]
    fn file_get_view<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        disp: *mut MPI_Offset,
        etype: *mut MPI_Datatype,
        filetype: *mut MPI_Datatype,
        datarep: *mut c_char,
    ) -> c_int
    where
        F: FnOnce(
            MPI_File,
            *mut MPI_Offset,
            *mut MPI_Datatype,
            *mut MPI_Datatype,
            *mut c_char,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, disp, etype, filetype, datarep) }
    }
    #[inline]
    fn file_read_at<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Offset, *mut c_void, c_int, MPI_Datatype, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, status) }
    }
    #[inline]
    fn file_read_at_all<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Offset, *mut c_void, c_int, MPI_Datatype, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write_at<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            MPI_File,
            MPI_Offset,
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut MPI_Status,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write_at_all<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            MPI_File,
            MPI_Offset,
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut MPI_Status,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, status) }
    }
    #[inline]
    fn file_iread_at<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            MPI_File,
            MPI_Offset,
            *mut c_void,
            c_int,
            MPI_Datatype,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iwrite_at<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            MPI_File,
            MPI_Offset,
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, request) }
    }
    #[inline]
    fn file_read<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, c_int, MPI_Datatype, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_read_all<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, c_int, MPI_Datatype, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, c_int, MPI_Datatype, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write_all<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, c_int, MPI_Datatype, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_iread<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, c_int, MPI_Datatype, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iwrite<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, c_int, MPI_Datatype, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
    #[inline]
    fn file_seek<F>(next_f: UnsafeBox<F>, fh: MPI_File, offset: MPI_Offset, whence: c_int) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Offset, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, whence) }
    }
    #[inline]
    fn file_get_position<F>(next_f: UnsafeBox<F>, fh: MPI_File, offset: *mut MPI_Offset) -> c_int
    where
        F: FnOnce(MPI_File, *mut MPI_Offset) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset) }
    }
    #[inline]
    fn file_get_byte_offset<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        disp: *mut MPI_Offset,
    ) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Offset, *mut MPI_Offset) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, disp) }
    }
    #[inline]
    fn file_read_shared<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, c_int, MPI_Datatype, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write_shared<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, c_int, MPI_Datatype, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_iread_shared<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, c_int, MPI_Datatype, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iwrite_shared<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, c_int, MPI_Datatype, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
    #[inline]
    fn file_read_ordered<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, c_int, MPI_Datatype, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write_ordered<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, c_int, MPI_Datatype, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_seek_shared<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        whence: c_int,
    ) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Offset, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, whence) }
    }
    #[inline]
    fn file_get_position_shared<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: *mut MPI_Offset,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut MPI_Offset) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset) }
    }
    #[inline]
    fn file_read_at_all_begin<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Offset, *mut c_void, c_int, MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype) }
    }
    #[inline]
    fn file_read_at_all_end<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_write_at_all_begin<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Offset, *const c_void, c_int, MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype) }
    }
    #[inline]
    fn file_write_at_all_end<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_read_all_begin<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, c_int, MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype) }
    }
    #[inline]
    fn file_read_all_end<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_write_all_begin<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, c_int, MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype) }
    }
    #[inline]
    fn file_write_all_end<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_read_ordered_begin<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, c_int, MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype) }
    }
    #[inline]
    fn file_read_ordered_end<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_write_ordered_begin<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, c_int, MPI_Datatype) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype) }
    }
    #[inline]
    fn file_write_ordered_end<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        status: *mut MPI_Status,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, *mut MPI_Status) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_get_type_extent<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        datatype: MPI_Datatype,
        extent: *mut MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(MPI_File, MPI_Datatype, *mut MPI_Aint) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, datatype, extent) }
    }
    #[inline]
    fn register_datarep<F>(
        next_f: UnsafeBox<F>,
        datarep: *const c_char,
        read_conversion_fn: MPI_Datarep_conversion_function,
        write_conversion_fn: MPI_Datarep_conversion_function,
        dtype_file_extent_fn: MPI_Datarep_extent_function,
        extra_state: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(
            *const c_char,
            MPI_Datarep_conversion_function,
            MPI_Datarep_conversion_function,
            MPI_Datarep_extent_function,
            *mut c_void,
        ) -> c_int,
    {
        unsafe {
            next_f.unwrap()(
                datarep,
                read_conversion_fn,
                write_conversion_fn,
                dtype_file_extent_fn,
                extra_state,
            )
        }
    }
    #[inline]
    fn file_set_atomicity<F>(next_f: UnsafeBox<F>, fh: MPI_File, flag: c_int) -> c_int
    where
        F: FnOnce(MPI_File, c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, flag) }
    }
    #[inline]
    fn file_get_atomicity<F>(next_f: UnsafeBox<F>, fh: MPI_File, flag: *mut c_int) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_int) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, flag) }
    }
    #[inline]
    fn file_sync<F>(next_f: UnsafeBox<F>, fh: MPI_File) -> c_int
    where
        F: FnOnce(MPI_File) -> c_int,
    {
        unsafe { next_f.unwrap()(fh) }
    }
    #[inline]
    fn file_iread_at_all<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            MPI_File,
            MPI_Offset,
            *mut c_void,
            c_int,
            MPI_Datatype,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iwrite_at_all<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        offset: MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            MPI_File,
            MPI_Offset,
            *const c_void,
            c_int,
            MPI_Datatype,
            *mut MPI_Request,
        ) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iread_all<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: MPI_Datatype,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *mut c_void, c_int, MPI_Datatype, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iwrite_all<F>(
        next_f: UnsafeBox<F>,
        fh: MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        request: *mut MPI_Request,
    ) -> c_int
    where
        F: FnOnce(MPI_File, *const c_void, c_int, MPI_Datatype, *mut MPI_Request) -> c_int,
    {
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
}

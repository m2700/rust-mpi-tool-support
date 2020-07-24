use std::{
    collections::HashMap,
    sync::{
        Mutex,
    },
};

use lazy_static::lazy_static;

use mpi_tool_layer::{RawMpiInterceptionLayer, UnsafeBox};
use pmpi_tool_creator::{install_pmpi_layer as install_mpi_layer, mpi_sys};

lazy_static! {
    static ref MPI_FN_COUNTER_MAP: Mutex<HashMap<&'static str, usize>> = Mutex::new(HashMap::new());
}

struct MyQmpiLayer;
impl RawMpiInterceptionLayer for MyQmpiLayer {
    #[inline]
    fn send<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Send")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm) }
    }
    #[inline]
    fn recv<F>(
        next_f: UnsafeBox<F>,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Recv")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, source, tag, comm, status) }
    }
    #[inline]
    fn get_count<F>(
        next_f: UnsafeBox<F>,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const mpi_sys::MPI_Status,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Get_count")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(status, datatype, count) }
    }
    #[inline]
    fn bsend<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Bsend")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm) }
    }
    #[inline]
    fn ssend<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ssend")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm) }
    }
    #[inline]
    fn rsend<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Rsend")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm) }
    }
    #[inline]
    fn buffer_attach<F>(
        next_f: UnsafeBox<F>,
        buffer: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_void, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Buffer_attach")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buffer, size) }
    }
    #[inline]
    fn buffer_detach<F>(
        next_f: UnsafeBox<F>,
        buffer_addr: *mut ::std::os::raw::c_void,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Buffer_detach")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buffer_addr, size) }
    }
    #[inline]
    fn isend<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Isend")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn ibsend<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ibsend")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn issend<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Issend")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn irsend<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Irsend")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn irecv<F>(
        next_f: UnsafeBox<F>,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Irecv")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, source, tag, comm, request) }
    }
    #[inline]
    fn wait<F>(
        next_f: UnsafeBox<F>,
        request: *mut mpi_sys::MPI_Request,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request, *mut mpi_sys::MPI_Status) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Wait")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(request, status) }
    }
    #[inline]
    fn test<F>(
        next_f: UnsafeBox<F>,
        request: *mut mpi_sys::MPI_Request,
        flag: *mut ::std::os::raw::c_int,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut mpi_sys::MPI_Request,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Test")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(request, flag, status) }
    }
    #[inline]
    fn request_free<F>(
        next_f: UnsafeBox<F>,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Request_free")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(request) }
    }
    #[inline]
    fn waitany<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        indx: *mut ::std::os::raw::c_int,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Request,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Waitany")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, array_of_requests, indx, status) }
    }
    #[inline]
    fn testany<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        indx: *mut ::std::os::raw::c_int,
        flag: *mut ::std::os::raw::c_int,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Request,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Testany")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, array_of_requests, indx, flag, status) }
    }
    #[inline]
    fn waitall<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Request,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Waitall")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, array_of_requests, array_of_statuses) }
    }
    #[inline]
    fn testall<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        flag: *mut ::std::os::raw::c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Request,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Testall")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, array_of_requests, flag, array_of_statuses) }
    }
    #[inline]
    fn waitsome<F>(
        next_f: UnsafeBox<F>,
        incount: ::std::os::raw::c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut ::std::os::raw::c_int,
        array_of_indices: *mut ::std::os::raw::c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Request,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Waitsome")
            .or_insert(0) += 1;
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
        incount: ::std::os::raw::c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut ::std::os::raw::c_int,
        array_of_indices: *mut ::std::os::raw::c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Request,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Testsome")
            .or_insert(0) += 1;
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
        source: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        flag: *mut ::std::os::raw::c_int,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Iprobe")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(source, tag, comm, flag, status) }
    }
    #[inline]
    fn probe<F>(
        next_f: UnsafeBox<F>,
        source: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Probe")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(source, tag, comm, status) }
    }
    #[inline]
    fn cancel<F>(next_f: UnsafeBox<F>, request: *mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Cancel")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(request) }
    }
    #[inline]
    fn test_cancelled<F>(
        next_f: UnsafeBox<F>,
        status: *const mpi_sys::MPI_Status,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*const mpi_sys::MPI_Status, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Test_cancelled")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(status, flag) }
    }
    #[inline]
    fn send_init<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Send_init")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn bsend_init<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Bsend_init")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn ssend_init<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ssend_init")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn rsend_init<F>(
        next_f: UnsafeBox<F>,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Rsend_init")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, dest, tag, comm, request) }
    }
    #[inline]
    fn recv_init<F>(
        next_f: UnsafeBox<F>,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Recv_init")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, source, tag, comm, request) }
    }
    #[inline]
    fn start<F>(next_f: UnsafeBox<F>, request: *mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Start")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(request) }
    }
    #[inline]
    fn startall<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, *mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Startall")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, array_of_requests) }
    }
    #[inline]
    fn sendrecv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        sendtag: ::std::os::raw::c_int,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        source: ::std::os::raw::c_int,
        recvtag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Sendrecv")
            .or_insert(0) += 1;
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
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: ::std::os::raw::c_int,
        sendtag: ::std::os::raw::c_int,
        source: ::std::os::raw::c_int,
        recvtag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Sendrecv_replace")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                buf, count, datatype, dest, sendtag, source, recvtag, comm, status,
            )
        }
    }
    #[inline]
    fn type_contiguous<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_contiguous")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, oldtype, newtype) }
    }
    #[inline]
    fn type_vector<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        blocklength: ::std::os::raw::c_int,
        stride: ::std::os::raw::c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_vector")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, blocklength, stride, oldtype, newtype) }
    }
    #[inline]
    fn type_hvector<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        blocklength: ::std::os::raw::c_int,
        stride: mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_hvector")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, blocklength, stride, oldtype, newtype) }
    }
    #[inline]
    fn type_indexed<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        array_of_blocklengths: *const ::std::os::raw::c_int,
        array_of_displacements: *const ::std::os::raw::c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_indexed")
            .or_insert(0) += 1;
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
        count: ::std::os::raw::c_int,
        array_of_blocklengths: *mut ::std::os::raw::c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Aint,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_hindexed")
            .or_insert(0) += 1;
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
        count: ::std::os::raw::c_int,
        array_of_blocklengths: *mut ::std::os::raw::c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        array_of_types: *mut mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_struct")
            .or_insert(0) += 1;
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
    fn address<F>(
        next_f: UnsafeBox<F>,
        location: *mut ::std::os::raw::c_void,
        address: *mut mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_void, *mut mpi_sys::MPI_Aint) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Address")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(location, address) }
    }
    #[inline]
    fn type_extent<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        extent: *mut mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Aint) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_extent")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, extent) }
    }
    #[inline]
    fn type_size<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_size")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, size) }
    }
    #[inline]
    fn type_lb<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        displacement: *mut mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Aint) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_lb")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, displacement) }
    }
    #[inline]
    fn type_ub<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        displacement: *mut mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Aint) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_ub")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, displacement) }
    }
    #[inline]
    fn type_commit<F>(
        next_f: UnsafeBox<F>,
        datatype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Datatype) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_commit")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype) }
    }
    #[inline]
    fn type_free<F>(
        next_f: UnsafeBox<F>,
        datatype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Datatype) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_free")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype) }
    }
    #[inline]
    fn get_elements<F>(
        next_f: UnsafeBox<F>,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const mpi_sys::MPI_Status,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Get_elements")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(status, datatype, count) }
    }
    #[inline]
    fn pack<F>(
        next_f: UnsafeBox<F>,
        inbuf: *const ::std::os::raw::c_void,
        incount: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        outbuf: *mut ::std::os::raw::c_void,
        outsize: ::std::os::raw::c_int,
        position: *mut ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Pack")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(inbuf, incount, datatype, outbuf, outsize, position, comm) }
    }
    #[inline]
    fn unpack<F>(
        next_f: UnsafeBox<F>,
        inbuf: *const ::std::os::raw::c_void,
        insize: ::std::os::raw::c_int,
        position: *mut ::std::os::raw::c_int,
        outbuf: *mut ::std::os::raw::c_void,
        outcount: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Unpack")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(inbuf, insize, position, outbuf, outcount, datatype, comm) }
    }
    #[inline]
    fn pack_size<F>(
        next_f: UnsafeBox<F>,
        incount: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Pack_size")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(incount, datatype, comm, size) }
    }
    #[inline]
    fn barrier<F>(next_f: UnsafeBox<F>, comm: mpi_sys::MPI_Comm) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Barrier")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm) }
    }
    #[inline]
    fn bcast<F>(
        next_f: UnsafeBox<F>,
        buffer: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Bcast")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buffer, count, datatype, root, comm) }
    }
    #[inline]
    fn gather<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Gather")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm,
            )
        }
    }
    #[inline]
    fn gatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        displs: *const ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Gatherv")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, root, comm,
            )
        }
    }
    #[inline]
    fn scatter<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Scatter")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm,
            )
        }
    }
    #[inline]
    fn scatterv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcounts: *const ::std::os::raw::c_int,
        displs: *const ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Scatterv")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcounts, displs, sendtype, recvbuf, recvcount, recvtype, root, comm,
            )
        }
    }
    #[inline]
    fn allgather<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Allgather")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
            )
        }
    }
    #[inline]
    fn allgatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        displs: *const ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Allgatherv")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm,
            )
        }
    }
    #[inline]
    fn alltoall<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Alltoall")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
            )
        }
    }
    #[inline]
    fn alltoallv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcounts: *const ::std::os::raw::c_int,
        sdispls: *const ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        rdispls: *const ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Alltoallv")
            .or_insert(0) += 1;
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
        sendbuf: *const ::std::os::raw::c_void,
        sendcounts: *const ::std::os::raw::c_int,
        sdispls: *const ::std::os::raw::c_int,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        rdispls: *const ::std::os::raw::c_int,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Alltoallw")
            .or_insert(0) += 1;
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
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Exscan")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm) }
    }
    #[inline]
    fn reduce<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Reduce")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, root, comm) }
    }
    #[inline]
    fn op_create<F>(
        next_f: UnsafeBox<F>,
        user_fn: mpi_sys::MPI_User_function,
        commute: ::std::os::raw::c_int,
        op: *mut mpi_sys::MPI_Op,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_User_function,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Op,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Op_create")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(user_fn, commute, op) }
    }
    #[inline]
    fn op_free<F>(next_f: UnsafeBox<F>, op: *mut mpi_sys::MPI_Op) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Op) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Op_free")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(op) }
    }
    #[inline]
    fn allreduce<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Allreduce")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm) }
    }
    #[inline]
    fn reduce_scatter<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Reduce_scatter")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, recvcounts, datatype, op, comm) }
    }
    #[inline]
    fn scan<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Scan")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm) }
    }
    #[inline]
    fn group_size<F>(
        next_f: UnsafeBox<F>,
        group: mpi_sys::MPI_Group,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_size")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group, size) }
    }
    #[inline]
    fn group_rank<F>(
        next_f: UnsafeBox<F>,
        group: mpi_sys::MPI_Group,
        rank: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_rank")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group, rank) }
    }
    #[inline]
    fn group_translate_ranks<F>(
        next_f: UnsafeBox<F>,
        group1: mpi_sys::MPI_Group,
        n: ::std::os::raw::c_int,
        ranks1: *const ::std::os::raw::c_int,
        group2: mpi_sys::MPI_Group,
        ranks2: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Group,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_translate_ranks")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group1, n, ranks1, group2, ranks2) }
    }
    #[inline]
    fn group_compare<F>(
        next_f: UnsafeBox<F>,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        result: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            mpi_sys::MPI_Group,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_compare")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group1, group2, result) }
    }
    #[inline]
    fn comm_group<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        group: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Group) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_group")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, group) }
    }
    #[inline]
    fn group_union<F>(
        next_f: UnsafeBox<F>,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            mpi_sys::MPI_Group,
            *mut mpi_sys::MPI_Group,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_union")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group1, group2, newgroup) }
    }
    #[inline]
    fn group_intersection<F>(
        next_f: UnsafeBox<F>,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            mpi_sys::MPI_Group,
            *mut mpi_sys::MPI_Group,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_intersection")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group1, group2, newgroup) }
    }
    #[inline]
    fn group_difference<F>(
        next_f: UnsafeBox<F>,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            mpi_sys::MPI_Group,
            *mut mpi_sys::MPI_Group,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_difference")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group1, group2, newgroup) }
    }
    #[inline]
    fn group_incl<F>(
        next_f: UnsafeBox<F>,
        group: mpi_sys::MPI_Group,
        n: ::std::os::raw::c_int,
        ranks: *const ::std::os::raw::c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Group,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_incl")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group, n, ranks, newgroup) }
    }
    #[inline]
    fn group_excl<F>(
        next_f: UnsafeBox<F>,
        group: mpi_sys::MPI_Group,
        n: ::std::os::raw::c_int,
        ranks: *const ::std::os::raw::c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Group,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_excl")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group, n, ranks, newgroup) }
    }
    #[inline]
    fn group_range_incl<F>(
        next_f: UnsafeBox<F>,
        group: mpi_sys::MPI_Group,
        n: ::std::os::raw::c_int,
        ranges: *mut [::std::os::raw::c_int; 3usize],
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            ::std::os::raw::c_int,
            *mut [::std::os::raw::c_int; 3usize],
            *mut mpi_sys::MPI_Group,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_range_incl")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group, n, ranges, newgroup) }
    }
    #[inline]
    fn group_range_excl<F>(
        next_f: UnsafeBox<F>,
        group: mpi_sys::MPI_Group,
        n: ::std::os::raw::c_int,
        ranges: *mut [::std::os::raw::c_int; 3usize],
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            ::std::os::raw::c_int,
            *mut [::std::os::raw::c_int; 3usize],
            *mut mpi_sys::MPI_Group,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_range_excl")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group, n, ranges, newgroup) }
    }
    #[inline]
    fn group_free<F>(next_f: UnsafeBox<F>, group: *mut mpi_sys::MPI_Group) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Group) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Group_free")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group) }
    }
    #[inline]
    fn comm_size<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_size")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, size) }
    }
    #[inline]
    fn comm_rank<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        rank: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_rank")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, rank) }
    }
    #[inline]
    fn comm_compare<F>(
        next_f: UnsafeBox<F>,
        comm1: mpi_sys::MPI_Comm,
        comm2: mpi_sys::MPI_Comm,
        result: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            mpi_sys::MPI_Comm,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_compare")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm1, comm2, result) }
    }
    #[inline]
    fn comm_dup<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Comm) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_dup")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, newcomm) }
    }
    #[inline]
    fn comm_dup_with_info<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            mpi_sys::MPI_Info,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_dup_with_info")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, info, newcomm) }
    }
    #[inline]
    fn comm_create<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            mpi_sys::MPI_Group,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_create")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, group, newcomm) }
    }
    #[inline]
    fn comm_split<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        color: ::std::os::raw::c_int,
        key: ::std::os::raw::c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_split")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, color, key, newcomm) }
    }
    #[inline]
    fn comm_free<F>(next_f: UnsafeBox<F>, comm: *mut mpi_sys::MPI_Comm) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Comm) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_free")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm) }
    }
    #[inline]
    fn comm_test_inter<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_test_inter")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, flag) }
    }
    #[inline]
    fn comm_remote_size<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_remote_size")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, size) }
    }
    #[inline]
    fn comm_remote_group<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        group: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Group) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_remote_group")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, group) }
    }
    #[inline]
    fn intercomm_create<F>(
        next_f: UnsafeBox<F>,
        local_comm: mpi_sys::MPI_Comm,
        local_leader: ::std::os::raw::c_int,
        peer_comm: mpi_sys::MPI_Comm,
        remote_leader: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        newintercomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Intercomm_create")
            .or_insert(0) += 1;
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
        intercomm: mpi_sys::MPI_Comm,
        high: ::std::os::raw::c_int,
        newintracomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Intercomm_merge")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(intercomm, high, newintracomm) }
    }
    #[inline]
    fn keyval_create<F>(
        next_f: UnsafeBox<F>,
        copy_fn: mpi_sys::MPI_Copy_function,
        delete_fn: mpi_sys::MPI_Delete_function,
        keyval: *mut ::std::os::raw::c_int,
        extra_state: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Copy_function,
            mpi_sys::MPI_Delete_function,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Keyval_create")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(copy_fn, delete_fn, keyval, extra_state) }
    }
    #[inline]
    fn keyval_free<F>(
        next_f: UnsafeBox<F>,
        keyval: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Keyval_free")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(keyval) }
    }
    #[inline]
    fn attr_put<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Attr_put")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, keyval, attribute_val) }
    }
    #[inline]
    fn attr_get<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Attr_get")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, keyval, attribute_val, flag) }
    }
    #[inline]
    fn attr_delete<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        keyval: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Attr_delete")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, keyval) }
    }
    #[inline]
    fn topo_test<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        status: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Topo_test")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, status) }
    }
    #[inline]
    fn cart_create<F>(
        next_f: UnsafeBox<F>,
        comm_old: mpi_sys::MPI_Comm,
        ndims: ::std::os::raw::c_int,
        dims: *const ::std::os::raw::c_int,
        periods: *const ::std::os::raw::c_int,
        reorder: ::std::os::raw::c_int,
        comm_cart: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Cart_create")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm_old, ndims, dims, periods, reorder, comm_cart) }
    }
    #[inline]
    fn dims_create<F>(
        next_f: UnsafeBox<F>,
        nnodes: ::std::os::raw::c_int,
        ndims: ::std::os::raw::c_int,
        dims: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Dims_create")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(nnodes, ndims, dims) }
    }
    #[inline]
    fn graph_create<F>(
        next_f: UnsafeBox<F>,
        comm_old: mpi_sys::MPI_Comm,
        nnodes: ::std::os::raw::c_int,
        indx: *const ::std::os::raw::c_int,
        edges: *const ::std::os::raw::c_int,
        reorder: ::std::os::raw::c_int,
        comm_graph: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Graph_create")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm_old, nnodes, indx, edges, reorder, comm_graph) }
    }
    #[inline]
    fn graphdims_get<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        nnodes: *mut ::std::os::raw::c_int,
        nedges: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Graphdims_get")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, nnodes, nedges) }
    }
    #[inline]
    fn graph_get<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        maxindex: ::std::os::raw::c_int,
        maxedges: ::std::os::raw::c_int,
        indx: *mut ::std::os::raw::c_int,
        edges: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Graph_get")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, maxindex, maxedges, indx, edges) }
    }
    #[inline]
    fn cartdim_get<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        ndims: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Cartdim_get")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, ndims) }
    }
    #[inline]
    fn cart_get<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        maxdims: ::std::os::raw::c_int,
        dims: *mut ::std::os::raw::c_int,
        periods: *mut ::std::os::raw::c_int,
        coords: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Cart_get")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, maxdims, dims, periods, coords) }
    }
    #[inline]
    fn cart_rank<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        coords: *const ::std::os::raw::c_int,
        rank: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            *const ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Cart_rank")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, coords, rank) }
    }
    #[inline]
    fn cart_coords<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        rank: ::std::os::raw::c_int,
        maxdims: ::std::os::raw::c_int,
        coords: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Cart_coords")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, rank, maxdims, coords) }
    }
    #[inline]
    fn graph_neighbors_count<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        rank: ::std::os::raw::c_int,
        nneighbors: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Graph_neighbors_count")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, rank, nneighbors) }
    }
    #[inline]
    fn graph_neighbors<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        rank: ::std::os::raw::c_int,
        maxneighbors: ::std::os::raw::c_int,
        neighbors: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Graph_neighbors")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, rank, maxneighbors, neighbors) }
    }
    #[inline]
    fn cart_shift<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        direction: ::std::os::raw::c_int,
        disp: ::std::os::raw::c_int,
        rank_source: *mut ::std::os::raw::c_int,
        rank_dest: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Cart_shift")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, direction, disp, rank_source, rank_dest) }
    }
    #[inline]
    fn cart_sub<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        remain_dims: *const ::std::os::raw::c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            *const ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Cart_sub")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, remain_dims, newcomm) }
    }
    #[inline]
    fn cart_map<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        ndims: ::std::os::raw::c_int,
        dims: *const ::std::os::raw::c_int,
        periods: *const ::std::os::raw::c_int,
        newrank: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Cart_map")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, ndims, dims, periods, newrank) }
    }
    #[inline]
    fn graph_map<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        nnodes: ::std::os::raw::c_int,
        indx: *const ::std::os::raw::c_int,
        edges: *const ::std::os::raw::c_int,
        newrank: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Graph_map")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, nnodes, indx, edges, newrank) }
    }
    #[inline]
    fn get_processor_name<F>(
        next_f: UnsafeBox<F>,
        name: *mut ::std::os::raw::c_char,
        resultlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_char, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Get_processor_name")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(name, resultlen) }
    }
    #[inline]
    fn get_version<F>(
        next_f: UnsafeBox<F>,
        version: *mut ::std::os::raw::c_int,
        subversion: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Get_version")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(version, subversion) }
    }
    #[inline]
    fn get_library_version<F>(
        next_f: UnsafeBox<F>,
        version: *mut ::std::os::raw::c_char,
        resultlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_char, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Get_library_version")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(version, resultlen) }
    }
    #[inline]
    fn errhandler_create<F>(
        next_f: UnsafeBox<F>,
        function: mpi_sys::MPI_Handler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Handler_function,
            *mut mpi_sys::MPI_Errhandler,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Errhandler_create")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(function, errhandler) }
    }
    #[inline]
    fn errhandler_set<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        errhandler: mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, mpi_sys::MPI_Errhandler) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Errhandler_set")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, errhandler) }
    }
    #[inline]
    fn errhandler_get<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Errhandler) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Errhandler_get")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, errhandler) }
    }
    #[inline]
    fn errhandler_free<F>(
        next_f: UnsafeBox<F>,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Errhandler) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Errhandler_free")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(errhandler) }
    }
    #[inline]
    fn error_string<F>(
        next_f: UnsafeBox<F>,
        errorcode: ::std::os::raw::c_int,
        string: *mut ::std::os::raw::c_char,
        resultlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_char,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Error_string")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(errorcode, string, resultlen) }
    }
    #[inline]
    fn error_class<F>(
        next_f: UnsafeBox<F>,
        errorcode: ::std::os::raw::c_int,
        errorclass: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Error_class")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(errorcode, errorclass) }
    }
    #[inline]
    fn wtime<F>(next_f: UnsafeBox<F>) -> f64
    where
        F: FnOnce() -> f64,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Wtime")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()() }
    }
    #[inline]
    fn wtick<F>(next_f: UnsafeBox<F>) -> f64
    where
        F: FnOnce() -> f64,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Wtick")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()() }
    }
    #[inline]
    fn init<F>(
        next_f: UnsafeBox<F>,
        argc: *mut ::std::os::raw::c_int,
        argv: *mut *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_int,
            *mut *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Init")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(argc, argv) }
    }
    #[inline]
    fn finalize<F>(next_f: UnsafeBox<F>) -> ::std::os::raw::c_int
    where
        F: FnOnce() -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Finalize")
            .or_insert(0) += 1;
        let res = unsafe { next_f.unwrap()() };
        dbg!(&*MPI_FN_COUNTER_MAP.lock().unwrap());
        res
    }
    #[inline]
    fn initialized<F>(
        next_f: UnsafeBox<F>,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Initialized")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(flag) }
    }
    #[inline]
    fn abort<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        errorcode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Abort")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, errorcode) }
    }
    #[inline]
    fn pcontrol<F>(next_f: UnsafeBox<F>, level: ::std::os::raw::c_int) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Pcontrol")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(level) }
    }
    #[inline]
    fn close_port<F>(
        next_f: UnsafeBox<F>,
        port_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Close_port")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(port_name) }
    }
    #[inline]
    fn comm_accept<F>(
        next_f: UnsafeBox<F>,
        port_name: *const ::std::os::raw::c_char,
        info: mpi_sys::MPI_Info,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_char,
            mpi_sys::MPI_Info,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_accept")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(port_name, info, root, comm, newcomm) }
    }
    #[inline]
    fn comm_connect<F>(
        next_f: UnsafeBox<F>,
        port_name: *const ::std::os::raw::c_char,
        info: mpi_sys::MPI_Info,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_char,
            mpi_sys::MPI_Info,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_connect")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(port_name, info, root, comm, newcomm) }
    }
    #[inline]
    fn comm_disconnect<F>(
        next_f: UnsafeBox<F>,
        comm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Comm) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_disconnect")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm) }
    }
    #[inline]
    fn comm_get_parent<F>(
        next_f: UnsafeBox<F>,
        parent: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Comm) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_get_parent")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(parent) }
    }
    #[inline]
    fn comm_join<F>(
        next_f: UnsafeBox<F>,
        fd: ::std::os::raw::c_int,
        intercomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, *mut mpi_sys::MPI_Comm) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_join")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fd, intercomm) }
    }
    #[inline]
    fn lookup_name<F>(
        next_f: UnsafeBox<F>,
        service_name: *const ::std::os::raw::c_char,
        info: mpi_sys::MPI_Info,
        port_name: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_char,
            mpi_sys::MPI_Info,
            *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Lookup_name")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(service_name, info, port_name) }
    }
    #[inline]
    fn open_port<F>(
        next_f: UnsafeBox<F>,
        info: mpi_sys::MPI_Info,
        port_name: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Open_port")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info, port_name) }
    }
    #[inline]
    fn publish_name<F>(
        next_f: UnsafeBox<F>,
        service_name: *const ::std::os::raw::c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_char,
            mpi_sys::MPI_Info,
            *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Publish_name")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(service_name, info, port_name) }
    }
    #[inline]
    fn unpublish_name<F>(
        next_f: UnsafeBox<F>,
        service_name: *const ::std::os::raw::c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_char,
            mpi_sys::MPI_Info,
            *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Unpublish_name")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(service_name, info, port_name) }
    }
    #[inline]
    fn comm_set_info<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        info: mpi_sys::MPI_Info,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_set_info")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, info) }
    }
    #[inline]
    fn comm_get_info<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        info: *mut mpi_sys::MPI_Info,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_get_info")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, info) }
    }
    #[inline]
    fn accumulate<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *const ::std::os::raw::c_void,
        origin_count: ::std::os::raw::c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: ::std::os::raw::c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: ::std::os::raw::c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Accumulate")
            .or_insert(0) += 1;
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
        origin_addr: *mut ::std::os::raw::c_void,
        origin_count: ::std::os::raw::c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: ::std::os::raw::c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: ::std::os::raw::c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP.lock().unwrap().entry("Get").or_insert(0) += 1;
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
        origin_addr: *const ::std::os::raw::c_void,
        origin_count: ::std::os::raw::c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: ::std::os::raw::c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: ::std::os::raw::c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP.lock().unwrap().entry("Put").or_insert(0) += 1;
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
    fn win_complete<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_complete")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_create<F>(
        next_f: UnsafeBox<F>,
        base: *mut ::std::os::raw::c_void,
        size: mpi_sys::MPI_Aint,
        disp_unit: ::std::os::raw::c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        win: *mut mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Info,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_create")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(base, size, disp_unit, info, comm, win) }
    }
    #[inline]
    fn win_fence<F>(
        next_f: UnsafeBox<F>,
        assert: ::std::os::raw::c_int,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_fence")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(assert, win) }
    }
    #[inline]
    fn win_free<F>(next_f: UnsafeBox<F>, win: *mut mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_free")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_get_group<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        group: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *mut mpi_sys::MPI_Group) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_get_group")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, group) }
    }
    #[inline]
    fn win_lock<F>(
        next_f: UnsafeBox<F>,
        lock_type: ::std::os::raw::c_int,
        rank: ::std::os::raw::c_int,
        assert: ::std::os::raw::c_int,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_lock")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(lock_type, rank, assert, win) }
    }
    #[inline]
    fn win_post<F>(
        next_f: UnsafeBox<F>,
        group: mpi_sys::MPI_Group,
        assert: ::std::os::raw::c_int,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_post")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group, assert, win) }
    }
    #[inline]
    fn win_start<F>(
        next_f: UnsafeBox<F>,
        group: mpi_sys::MPI_Group,
        assert: ::std::os::raw::c_int,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Group,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_start")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(group, assert, win) }
    }
    #[inline]
    fn win_test<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_test")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, flag) }
    }
    #[inline]
    fn win_unlock<F>(
        next_f: UnsafeBox<F>,
        rank: ::std::os::raw::c_int,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_unlock")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(rank, win) }
    }
    #[inline]
    fn win_wait<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_wait")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_allocate<F>(
        next_f: UnsafeBox<F>,
        size: mpi_sys::MPI_Aint,
        disp_unit: ::std::os::raw::c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        baseptr: *mut ::std::os::raw::c_void,
        win: *mut mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Info,
            mpi_sys::MPI_Comm,
            *mut ::std::os::raw::c_void,
            *mut mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_allocate")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(size, disp_unit, info, comm, baseptr, win) }
    }
    #[inline]
    fn win_allocate_shared<F>(
        next_f: UnsafeBox<F>,
        size: mpi_sys::MPI_Aint,
        disp_unit: ::std::os::raw::c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        baseptr: *mut ::std::os::raw::c_void,
        win: *mut mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Info,
            mpi_sys::MPI_Comm,
            *mut ::std::os::raw::c_void,
            *mut mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_allocate_shared")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(size, disp_unit, info, comm, baseptr, win) }
    }
    #[inline]
    fn win_shared_query<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        rank: ::std::os::raw::c_int,
        size: *mut mpi_sys::MPI_Aint,
        disp_unit: *mut ::std::os::raw::c_int,
        baseptr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Win,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Aint,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_shared_query")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, rank, size, disp_unit, baseptr) }
    }
    #[inline]
    fn win_create_dynamic<F>(
        next_f: UnsafeBox<F>,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        win: *mut mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Info,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_create_dynamic")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info, comm, win) }
    }
    #[inline]
    fn win_attach<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        base: *mut ::std::os::raw::c_void,
        size: mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Win,
            *mut ::std::os::raw::c_void,
            mpi_sys::MPI_Aint,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_attach")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, base, size) }
    }
    #[inline]
    fn win_detach<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        base: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *const ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_detach")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, base) }
    }
    #[inline]
    fn win_get_info<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        info_used: *mut mpi_sys::MPI_Info,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_get_info")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, info_used) }
    }
    #[inline]
    fn win_set_info<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        info: mpi_sys::MPI_Info,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_set_info")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, info) }
    }
    #[inline]
    fn get_accumulate<F>(
        next_f: UnsafeBox<F>,
        origin_addr: *const ::std::os::raw::c_void,
        origin_count: ::std::os::raw::c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        result_addr: *mut ::std::os::raw::c_void,
        result_count: ::std::os::raw::c_int,
        result_datatype: mpi_sys::MPI_Datatype,
        target_rank: ::std::os::raw::c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: ::std::os::raw::c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Get_accumulate")
            .or_insert(0) += 1;
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
        origin_addr: *const ::std::os::raw::c_void,
        result_addr: *mut ::std::os::raw::c_void,
        datatype: mpi_sys::MPI_Datatype,
        target_rank: ::std::os::raw::c_int,
        target_disp: mpi_sys::MPI_Aint,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Fetch_and_op")
            .or_insert(0) += 1;
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
        origin_addr: *const ::std::os::raw::c_void,
        compare_addr: *const ::std::os::raw::c_void,
        result_addr: *mut ::std::os::raw::c_void,
        datatype: mpi_sys::MPI_Datatype,
        target_rank: ::std::os::raw::c_int,
        target_disp: mpi_sys::MPI_Aint,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Win,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Compare_and_swap")
            .or_insert(0) += 1;
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
        origin_addr: *const ::std::os::raw::c_void,
        origin_count: ::std::os::raw::c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: ::std::os::raw::c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: ::std::os::raw::c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Win,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Rput")
            .or_insert(0) += 1;
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
        origin_addr: *mut ::std::os::raw::c_void,
        origin_count: ::std::os::raw::c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: ::std::os::raw::c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: ::std::os::raw::c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Win,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Rget")
            .or_insert(0) += 1;
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
        origin_addr: *const ::std::os::raw::c_void,
        origin_count: ::std::os::raw::c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: ::std::os::raw::c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: ::std::os::raw::c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Win,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Raccumulate")
            .or_insert(0) += 1;
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
        origin_addr: *const ::std::os::raw::c_void,
        origin_count: ::std::os::raw::c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        result_addr: *mut ::std::os::raw::c_void,
        result_count: ::std::os::raw::c_int,
        result_datatype: mpi_sys::MPI_Datatype,
        target_rank: ::std::os::raw::c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: ::std::os::raw::c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Win,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Rget_accumulate")
            .or_insert(0) += 1;
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
    fn win_lock_all<F>(
        next_f: UnsafeBox<F>,
        assert: ::std::os::raw::c_int,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_lock_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(assert, win) }
    }
    #[inline]
    fn win_unlock_all<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_unlock_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_flush<F>(
        next_f: UnsafeBox<F>,
        rank: ::std::os::raw::c_int,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_flush")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(rank, win) }
    }
    #[inline]
    fn win_flush_all<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_flush_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_flush_local<F>(
        next_f: UnsafeBox<F>,
        rank: ::std::os::raw::c_int,
        win: mpi_sys::MPI_Win,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_flush_local")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(rank, win) }
    }
    #[inline]
    fn win_flush_local_all<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_flush_local_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_sync<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_sync")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn add_error_class<F>(
        next_f: UnsafeBox<F>,
        errorclass: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Add_error_class")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(errorclass) }
    }
    #[inline]
    fn add_error_code<F>(
        next_f: UnsafeBox<F>,
        errorclass: ::std::os::raw::c_int,
        errorcode: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Add_error_code")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(errorclass, errorcode) }
    }
    #[inline]
    fn add_error_string<F>(
        next_f: UnsafeBox<F>,
        errorcode: ::std::os::raw::c_int,
        string: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Add_error_string")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(errorcode, string) }
    }
    #[inline]
    fn comm_call_errhandler<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        errorcode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_call_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, errorcode) }
    }
    #[inline]
    fn comm_create_keyval<F>(
        next_f: UnsafeBox<F>,
        comm_copy_attr_fn: mpi_sys::MPI_Comm_copy_attr_function,
        comm_delete_attr_fn: mpi_sys::MPI_Comm_delete_attr_function,
        comm_keyval: *mut ::std::os::raw::c_int,
        extra_state: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm_copy_attr_function,
            mpi_sys::MPI_Comm_delete_attr_function,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_create_keyval")
            .or_insert(0) += 1;
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
    fn comm_delete_attr<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_delete_attr")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, comm_keyval) }
    }
    #[inline]
    fn comm_free_keyval<F>(
        next_f: UnsafeBox<F>,
        comm_keyval: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_free_keyval")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm_keyval) }
    }
    #[inline]
    fn comm_get_attr<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_get_attr")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, comm_keyval, attribute_val, flag) }
    }
    #[inline]
    fn comm_get_name<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        comm_name: *mut ::std::os::raw::c_char,
        resultlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            *mut ::std::os::raw::c_char,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_get_name")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, comm_name, resultlen) }
    }
    #[inline]
    fn comm_set_attr<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_set_attr")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, comm_keyval, attribute_val) }
    }
    #[inline]
    fn comm_set_name<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        comm_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_set_name")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, comm_name) }
    }
    #[inline]
    fn file_call_errhandler<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        errorcode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_call_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, errorcode) }
    }
    #[inline]
    fn grequest_complete<F>(
        next_f: UnsafeBox<F>,
        request: mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Request) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Grequest_complete")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(request) }
    }
    #[inline]
    fn grequest_start<F>(
        next_f: UnsafeBox<F>,
        query_fn: mpi_sys::MPI_Grequest_query_function,
        free_fn: mpi_sys::MPI_Grequest_free_function,
        cancel_fn: mpi_sys::MPI_Grequest_cancel_function,
        extra_state: *mut ::std::os::raw::c_void,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Grequest_query_function,
            mpi_sys::MPI_Grequest_free_function,
            mpi_sys::MPI_Grequest_cancel_function,
            *mut ::std::os::raw::c_void,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Grequest_start")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(query_fn, free_fn, cancel_fn, extra_state, request) }
    }
    #[inline]
    fn init_thread<F>(
        next_f: UnsafeBox<F>,
        argc: *mut ::std::os::raw::c_int,
        argv: *mut *mut *mut ::std::os::raw::c_char,
        required: ::std::os::raw::c_int,
        provided: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_int,
            *mut *mut *mut ::std::os::raw::c_char,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Init_thread")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(argc, argv, required, provided) }
    }
    #[inline]
    fn is_thread_main<F>(
        next_f: UnsafeBox<F>,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Is_thread_main")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(flag) }
    }
    #[inline]
    fn query_thread<F>(
        next_f: UnsafeBox<F>,
        provided: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Query_thread")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(provided) }
    }
    #[inline]
    fn status_set_cancelled<F>(
        next_f: UnsafeBox<F>,
        status: *mut mpi_sys::MPI_Status,
        flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Status, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Status_set_cancelled")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(status, flag) }
    }
    #[inline]
    fn status_set_elements<F>(
        next_f: UnsafeBox<F>,
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut mpi_sys::MPI_Status,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Status_set_elements")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(status, datatype, count) }
    }
    #[inline]
    fn type_create_keyval<F>(
        next_f: UnsafeBox<F>,
        type_copy_attr_fn: mpi_sys::MPI_Type_copy_attr_function,
        type_delete_attr_fn: mpi_sys::MPI_Type_delete_attr_function,
        type_keyval: *mut ::std::os::raw::c_int,
        extra_state: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Type_copy_attr_function,
            mpi_sys::MPI_Type_delete_attr_function,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_keyval")
            .or_insert(0) += 1;
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
        datatype: mpi_sys::MPI_Datatype,
        type_keyval: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_delete_attr")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, type_keyval) }
    }
    #[inline]
    fn type_dup<F>(
        next_f: UnsafeBox<F>,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Datatype) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_dup")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(oldtype, newtype) }
    }
    #[inline]
    fn type_free_keyval<F>(
        next_f: UnsafeBox<F>,
        type_keyval: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_free_keyval")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(type_keyval) }
    }
    #[inline]
    fn type_get_attr<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        type_keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_get_attr")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, type_keyval, attribute_val, flag) }
    }
    #[inline]
    fn type_get_contents<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        max_integers: ::std::os::raw::c_int,
        max_addresses: ::std::os::raw::c_int,
        max_datatypes: ::std::os::raw::c_int,
        array_of_integers: *mut ::std::os::raw::c_int,
        array_of_addresses: *mut mpi_sys::MPI_Aint,
        array_of_datatypes: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_get_contents")
            .or_insert(0) += 1;
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
        datatype: mpi_sys::MPI_Datatype,
        num_integers: *mut ::std::os::raw::c_int,
        num_addresses: *mut ::std::os::raw::c_int,
        num_datatypes: *mut ::std::os::raw::c_int,
        combiner: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_get_envelope")
            .or_insert(0) += 1;
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
        datatype: mpi_sys::MPI_Datatype,
        type_name: *mut ::std::os::raw::c_char,
        resultlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_char,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_get_name")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, type_name, resultlen) }
    }
    #[inline]
    fn type_set_attr<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        type_keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_set_attr")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, type_keyval, attribute_val) }
    }
    #[inline]
    fn type_set_name<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        type_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_set_name")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, type_name) }
    }
    #[inline]
    fn type_match_size<F>(
        next_f: UnsafeBox<F>,
        typeclass: ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
        datatype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_match_size")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(typeclass, size, datatype) }
    }
    #[inline]
    fn win_call_errhandler<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        errorcode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_call_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, errorcode) }
    }
    #[inline]
    fn win_create_keyval<F>(
        next_f: UnsafeBox<F>,
        win_copy_attr_fn: mpi_sys::MPI_Win_copy_attr_function,
        win_delete_attr_fn: mpi_sys::MPI_Win_delete_attr_function,
        win_keyval: *mut ::std::os::raw::c_int,
        extra_state: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Win_copy_attr_function,
            mpi_sys::MPI_Win_delete_attr_function,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_create_keyval")
            .or_insert(0) += 1;
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
    fn win_delete_attr<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        win_keyval: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_delete_attr")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, win_keyval) }
    }
    #[inline]
    fn win_free_keyval<F>(
        next_f: UnsafeBox<F>,
        win_keyval: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_free_keyval")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win_keyval) }
    }
    #[inline]
    fn win_get_attr<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        win_keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Win,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_get_attr")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, win_keyval, attribute_val, flag) }
    }
    #[inline]
    fn win_get_name<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        win_name: *mut ::std::os::raw::c_char,
        resultlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Win,
            *mut ::std::os::raw::c_char,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_get_name")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, win_name, resultlen) }
    }
    #[inline]
    fn win_set_attr<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        win_keyval: ::std::os::raw::c_int,
        attribute_val: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Win,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_set_attr")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, win_keyval, attribute_val) }
    }
    #[inline]
    fn win_set_name<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        win_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_set_name")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, win_name) }
    }
    #[inline]
    fn alloc_mem<F>(
        next_f: UnsafeBox<F>,
        size: mpi_sys::MPI_Aint,
        info: mpi_sys::MPI_Info,
        baseptr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Info,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Alloc_mem")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(size, info, baseptr) }
    }
    #[inline]
    fn comm_create_errhandler<F>(
        next_f: UnsafeBox<F>,
        comm_errhandler_fn: mpi_sys::MPI_Comm_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm_errhandler_function,
            *mut mpi_sys::MPI_Errhandler,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_create_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm_errhandler_fn, errhandler) }
    }
    #[inline]
    fn comm_get_errhandler<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Errhandler) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_get_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, errhandler) }
    }
    #[inline]
    fn comm_set_errhandler<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        errhandler: mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, mpi_sys::MPI_Errhandler) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_set_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, errhandler) }
    }
    #[inline]
    fn file_create_errhandler<F>(
        next_f: UnsafeBox<F>,
        file_errhandler_fn: mpi_sys::MPI_File_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File_errhandler_function,
            *mut mpi_sys::MPI_Errhandler,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_create_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(file_errhandler_fn, errhandler) }
    }
    #[inline]
    fn file_get_errhandler<F>(
        next_f: UnsafeBox<F>,
        file: mpi_sys::MPI_File,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Errhandler) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(file, errhandler) }
    }
    #[inline]
    fn file_set_errhandler<F>(
        next_f: UnsafeBox<F>,
        file: mpi_sys::MPI_File,
        errhandler: mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Errhandler) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_set_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(file, errhandler) }
    }
    #[inline]
    fn finalized<F>(next_f: UnsafeBox<F>, flag: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Finalized")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(flag) }
    }
    #[inline]
    fn free_mem<F>(next_f: UnsafeBox<F>, base: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Free_mem")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(base) }
    }
    #[inline]
    fn get_address<F>(
        next_f: UnsafeBox<F>,
        location: *const ::std::os::raw::c_void,
        address: *mut mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*const ::std::os::raw::c_void, *mut mpi_sys::MPI_Aint) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Get_address")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(location, address) }
    }
    #[inline]
    fn info_create<F>(next_f: UnsafeBox<F>, info: *mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Info_create")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info) }
    }
    #[inline]
    fn info_delete<F>(
        next_f: UnsafeBox<F>,
        info: mpi_sys::MPI_Info,
        key: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Info_delete")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info, key) }
    }
    #[inline]
    fn info_dup<F>(
        next_f: UnsafeBox<F>,
        info: mpi_sys::MPI_Info,
        newinfo: *mut mpi_sys::MPI_Info,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, *mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Info_dup")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info, newinfo) }
    }
    #[inline]
    fn info_free<F>(next_f: UnsafeBox<F>, info: *mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Info_free")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info) }
    }
    #[inline]
    fn info_get<F>(
        next_f: UnsafeBox<F>,
        info: mpi_sys::MPI_Info,
        key: *const ::std::os::raw::c_char,
        valuelen: ::std::os::raw::c_int,
        value: *mut ::std::os::raw::c_char,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Info,
            *const ::std::os::raw::c_char,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_char,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Info_get")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info, key, valuelen, value, flag) }
    }
    #[inline]
    fn info_get_nkeys<F>(
        next_f: UnsafeBox<F>,
        info: mpi_sys::MPI_Info,
        nkeys: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Info_get_nkeys")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info, nkeys) }
    }
    #[inline]
    fn info_get_nthkey<F>(
        next_f: UnsafeBox<F>,
        info: mpi_sys::MPI_Info,
        n: ::std::os::raw::c_int,
        key: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Info,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Info_get_nthkey")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info, n, key) }
    }
    #[inline]
    fn info_get_valuelen<F>(
        next_f: UnsafeBox<F>,
        info: mpi_sys::MPI_Info,
        key: *const ::std::os::raw::c_char,
        valuelen: *mut ::std::os::raw::c_int,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Info,
            *const ::std::os::raw::c_char,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Info_get_valuelen")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info, key, valuelen, flag) }
    }
    #[inline]
    fn info_set<F>(
        next_f: UnsafeBox<F>,
        info: mpi_sys::MPI_Info,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Info,
            *const ::std::os::raw::c_char,
            *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Info_set")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(info, key, value) }
    }
    #[inline]
    fn pack_external<F>(
        next_f: UnsafeBox<F>,
        datarep: *const ::std::os::raw::c_char,
        inbuf: *const ::std::os::raw::c_void,
        incount: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        outbuf: *mut ::std::os::raw::c_void,
        outsize: mpi_sys::MPI_Aint,
        position: *mut mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_char,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Aint,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Pack_external")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datarep, inbuf, incount, datatype, outbuf, outsize, position) }
    }
    #[inline]
    fn pack_external_size<F>(
        next_f: UnsafeBox<F>,
        datarep: *const ::std::os::raw::c_char,
        incount: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        size: *mut mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_char,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Aint,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Pack_external_size")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datarep, incount, datatype, size) }
    }
    #[inline]
    fn request_get_status<F>(
        next_f: UnsafeBox<F>,
        request: mpi_sys::MPI_Request,
        flag: *mut ::std::os::raw::c_int,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Request,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Request_get_status")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(request, flag, status) }
    }
    #[inline]
    fn type_create_darray<F>(
        next_f: UnsafeBox<F>,
        size: ::std::os::raw::c_int,
        rank: ::std::os::raw::c_int,
        ndims: ::std::os::raw::c_int,
        array_of_gsizes: *const ::std::os::raw::c_int,
        array_of_distribs: *const ::std::os::raw::c_int,
        array_of_dargs: *const ::std::os::raw::c_int,
        array_of_psizes: *const ::std::os::raw::c_int,
        order: ::std::os::raw::c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_darray")
            .or_insert(0) += 1;
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
        count: ::std::os::raw::c_int,
        array_of_blocklengths: *const ::std::os::raw::c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const mpi_sys::MPI_Aint,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_hindexed")
            .or_insert(0) += 1;
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
        count: ::std::os::raw::c_int,
        blocklength: ::std::os::raw::c_int,
        stride: mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_hvector")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, blocklength, stride, oldtype, newtype) }
    }
    #[inline]
    fn type_create_indexed_block<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        blocklength: ::std::os::raw::c_int,
        array_of_displacements: *const ::std::os::raw::c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_indexed_block")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, blocklength, array_of_displacements, oldtype, newtype) }
    }
    #[inline]
    fn type_create_hindexed_block<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        blocklength: ::std::os::raw::c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *const mpi_sys::MPI_Aint,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_hindexed_block")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(count, blocklength, array_of_displacements, oldtype, newtype) }
    }
    #[inline]
    fn type_create_resized<F>(
        next_f: UnsafeBox<F>,
        oldtype: mpi_sys::MPI_Datatype,
        lb: mpi_sys::MPI_Aint,
        extent: mpi_sys::MPI_Aint,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_resized")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(oldtype, lb, extent, newtype) }
    }
    #[inline]
    fn type_create_struct<F>(
        next_f: UnsafeBox<F>,
        count: ::std::os::raw::c_int,
        array_of_blocklengths: *const ::std::os::raw::c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        array_of_types: *const mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const mpi_sys::MPI_Aint,
            *const mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_struct")
            .or_insert(0) += 1;
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
        ndims: ::std::os::raw::c_int,
        array_of_sizes: *const ::std::os::raw::c_int,
        array_of_subsizes: *const ::std::os::raw::c_int,
        array_of_starts: *const ::std::os::raw::c_int,
        order: ::std::os::raw::c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_subarray")
            .or_insert(0) += 1;
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
        datatype: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Aint,
        extent: *mut mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Aint,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_get_extent")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, lb, extent) }
    }
    #[inline]
    fn type_get_true_extent<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Aint,
        true_extent: *mut mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Aint,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_get_true_extent")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, true_lb, true_extent) }
    }
    #[inline]
    fn unpack_external<F>(
        next_f: UnsafeBox<F>,
        datarep: *const ::std::os::raw::c_char,
        inbuf: *const ::std::os::raw::c_void,
        insize: mpi_sys::MPI_Aint,
        position: *mut mpi_sys::MPI_Aint,
        outbuf: *mut ::std::os::raw::c_void,
        outcount: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_char,
            *const ::std::os::raw::c_void,
            mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Aint,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Unpack_external")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datarep, inbuf, insize, position, outbuf, outcount, datatype) }
    }
    #[inline]
    fn win_create_errhandler<F>(
        next_f: UnsafeBox<F>,
        win_errhandler_fn: mpi_sys::MPI_Win_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Win_errhandler_function,
            *mut mpi_sys::MPI_Errhandler,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_create_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win_errhandler_fn, errhandler) }
    }
    #[inline]
    fn win_get_errhandler<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *mut mpi_sys::MPI_Errhandler) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_get_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, errhandler) }
    }
    #[inline]
    fn win_set_errhandler<F>(
        next_f: UnsafeBox<F>,
        win: mpi_sys::MPI_Win,
        errhandler: mpi_sys::MPI_Errhandler,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, mpi_sys::MPI_Errhandler) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Win_set_errhandler")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(win, errhandler) }
    }
    #[inline]
    fn type_create_f90_integer<F>(
        next_f: UnsafeBox<F>,
        range: ::std::os::raw::c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int, *mut mpi_sys::MPI_Datatype) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_f90_integer")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(range, newtype) }
    }
    #[inline]
    fn type_create_f90_real<F>(
        next_f: UnsafeBox<F>,
        precision: ::std::os::raw::c_int,
        range: ::std::os::raw::c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_f90_real")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(precision, range, newtype) }
    }
    #[inline]
    fn type_create_f90_complex<F>(
        next_f: UnsafeBox<F>,
        precision: ::std::os::raw::c_int,
        range: ::std::os::raw::c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_create_f90_complex")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(precision, range, newtype) }
    }
    #[inline]
    fn reduce_local<F>(
        next_f: UnsafeBox<F>,
        inbuf: *const ::std::os::raw::c_void,
        inoutbuf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Reduce_local")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(inbuf, inoutbuf, count, datatype, op) }
    }
    #[inline]
    fn op_commutative<F>(
        next_f: UnsafeBox<F>,
        op: mpi_sys::MPI_Op,
        commute: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Op, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Op_commutative")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(op, commute) }
    }
    #[inline]
    fn reduce_scatter_block<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Reduce_scatter_block")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, recvcount, datatype, op, comm) }
    }
    #[inline]
    fn dist_graph_create_adjacent<F>(
        next_f: UnsafeBox<F>,
        comm_old: mpi_sys::MPI_Comm,
        indegree: ::std::os::raw::c_int,
        sources: *const ::std::os::raw::c_int,
        sourceweights: *const ::std::os::raw::c_int,
        outdegree: ::std::os::raw::c_int,
        destinations: *const ::std::os::raw::c_int,
        destweights: *const ::std::os::raw::c_int,
        info: mpi_sys::MPI_Info,
        reorder: ::std::os::raw::c_int,
        comm_dist_graph: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Info,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Dist_graph_create_adjacent")
            .or_insert(0) += 1;
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
        comm_old: mpi_sys::MPI_Comm,
        n: ::std::os::raw::c_int,
        sources: *const ::std::os::raw::c_int,
        degrees: *const ::std::os::raw::c_int,
        destinations: *const ::std::os::raw::c_int,
        weights: *const ::std::os::raw::c_int,
        info: mpi_sys::MPI_Info,
        reorder: ::std::os::raw::c_int,
        comm_dist_graph: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Info,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Dist_graph_create")
            .or_insert(0) += 1;
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
        comm: mpi_sys::MPI_Comm,
        indegree: *mut ::std::os::raw::c_int,
        outdegree: *mut ::std::os::raw::c_int,
        weighted: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Dist_graph_neighbors_count")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, indegree, outdegree, weighted) }
    }
    #[inline]
    fn dist_graph_neighbors<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        maxindegree: ::std::os::raw::c_int,
        sources: *mut ::std::os::raw::c_int,
        sourceweights: *mut ::std::os::raw::c_int,
        maxoutdegree: ::std::os::raw::c_int,
        destinations: *mut ::std::os::raw::c_int,
        destweights: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
            *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Dist_graph_neighbors")
            .or_insert(0) += 1;
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
        source: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        flag: *mut ::std::os::raw::c_int,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Message,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Improbe")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(source, tag, comm, flag, message, status) }
    }
    #[inline]
    fn imrecv<F>(
        next_f: UnsafeBox<F>,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        message: *mut mpi_sys::MPI_Message,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Message,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Imrecv")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, message, request) }
    }
    #[inline]
    fn mprobe<F>(
        next_f: UnsafeBox<F>,
        source: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Message,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Mprobe")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(source, tag, comm, message, status) }
    }
    #[inline]
    fn mrecv<F>(
        next_f: UnsafeBox<F>,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Message,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Mrecv")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buf, count, datatype, message, status) }
    }
    #[inline]
    fn comm_idup<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_idup")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, newcomm, request) }
    }
    #[inline]
    fn ibarrier<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ibarrier")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, request) }
    }
    #[inline]
    fn ibcast<F>(
        next_f: UnsafeBox<F>,
        buffer: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ibcast")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(buffer, count, datatype, root, comm, request) }
    }
    #[inline]
    fn igather<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Igather")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm, request,
            )
        }
    }
    #[inline]
    fn igatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        displs: *const ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Igatherv")
            .or_insert(0) += 1;
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
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Iscatter")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm, request,
            )
        }
    }
    #[inline]
    fn iscatterv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcounts: *const ::std::os::raw::c_int,
        displs: *const ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Iscatterv")
            .or_insert(0) += 1;
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
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Iallgather")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn iallgatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        displs: *const ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Iallgatherv")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn ialltoall<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ialltoall")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn ialltoallv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcounts: *const ::std::os::raw::c_int,
        sdispls: *const ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        rdispls: *const ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ialltoallv")
            .or_insert(0) += 1;
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
        sendbuf: *const ::std::os::raw::c_void,
        sendcounts: *const ::std::os::raw::c_int,
        sdispls: *const ::std::os::raw::c_int,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        rdispls: *const ::std::os::raw::c_int,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            *const mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ialltoallw")
            .or_insert(0) += 1;
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
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        root: ::std::os::raw::c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ireduce")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, root, comm, request) }
    }
    #[inline]
    fn iallreduce<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Iallreduce")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm, request) }
    }
    #[inline]
    fn ireduce_scatter<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ireduce_scatter")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, recvcounts, datatype, op, comm, request) }
    }
    #[inline]
    fn ireduce_scatter_block<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ireduce_scatter_block")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, recvcount, datatype, op, comm, request) }
    }
    #[inline]
    fn iscan<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Iscan")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm, request) }
    }
    #[inline]
    fn iexscan<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        recvbuf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Iexscan")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(sendbuf, recvbuf, count, datatype, op, comm, request) }
    }
    #[inline]
    fn ineighbor_allgather<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ineighbor_allgather")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn ineighbor_allgatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        displs: *const ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ineighbor_allgatherv")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn ineighbor_alltoall<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ineighbor_alltoall")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
            )
        }
    }
    #[inline]
    fn ineighbor_alltoallv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcounts: *const ::std::os::raw::c_int,
        sdispls: *const ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        rdispls: *const ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ineighbor_alltoallv")
            .or_insert(0) += 1;
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
        sendbuf: *const ::std::os::raw::c_void,
        sendcounts: *const ::std::os::raw::c_int,
        sdispls: *const mpi_sys::MPI_Aint,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        rdispls: *const mpi_sys::MPI_Aint,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const mpi_sys::MPI_Aint,
            *const mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const mpi_sys::MPI_Aint,
            *const mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Ineighbor_alltoallw")
            .or_insert(0) += 1;
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
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Neighbor_allgather")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
            )
        }
    }
    #[inline]
    fn neighbor_allgatherv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        displs: *const ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Neighbor_allgatherv")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm,
            )
        }
    }
    #[inline]
    fn neighbor_alltoall<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcount: ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcount: ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Neighbor_alltoall")
            .or_insert(0) += 1;
        unsafe {
            next_f.unwrap()(
                sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
            )
        }
    }
    #[inline]
    fn neighbor_alltoallv<F>(
        next_f: UnsafeBox<F>,
        sendbuf: *const ::std::os::raw::c_void,
        sendcounts: *const ::std::os::raw::c_int,
        sdispls: *const ::std::os::raw::c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        rdispls: *const ::std::os::raw::c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Neighbor_alltoallv")
            .or_insert(0) += 1;
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
        sendbuf: *const ::std::os::raw::c_void,
        sendcounts: *const ::std::os::raw::c_int,
        sdispls: *const mpi_sys::MPI_Aint,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut ::std::os::raw::c_void,
        recvcounts: *const ::std::os::raw::c_int,
        rdispls: *const mpi_sys::MPI_Aint,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const mpi_sys::MPI_Aint,
            *const mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_void,
            *const ::std::os::raw::c_int,
            *const mpi_sys::MPI_Aint,
            *const mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Neighbor_alltoallw")
            .or_insert(0) += 1;
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
        comm: mpi_sys::MPI_Comm,
        split_type: ::std::os::raw::c_int,
        key: ::std::os::raw::c_int,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            ::std::os::raw::c_int,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Info,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_split_type")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, split_type, key, info, newcomm) }
    }
    #[inline]
    fn get_elements_x<F>(
        next_f: UnsafeBox<F>,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut mpi_sys::MPI_Count,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const mpi_sys::MPI_Status,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Count,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Get_elements_x")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(status, datatype, count) }
    }
    #[inline]
    fn status_set_elements_x<F>(
        next_f: UnsafeBox<F>,
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: mpi_sys::MPI_Count,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *mut mpi_sys::MPI_Status,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Count,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Status_set_elements_x")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(status, datatype, count) }
    }
    #[inline]
    fn type_get_extent_x<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Count,
        extent: *mut mpi_sys::MPI_Count,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Count,
            *mut mpi_sys::MPI_Count,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_get_extent_x")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, lb, extent) }
    }
    #[inline]
    fn type_get_true_extent_x<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Count,
        extent: *mut mpi_sys::MPI_Count,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Count,
            *mut mpi_sys::MPI_Count,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_get_true_extent_x")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, lb, extent) }
    }
    #[inline]
    fn type_size_x<F>(
        next_f: UnsafeBox<F>,
        datatype: mpi_sys::MPI_Datatype,
        size: *mut mpi_sys::MPI_Count,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Count) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Type_size_x")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(datatype, size) }
    }
    #[inline]
    fn comm_create_group<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        tag: ::std::os::raw::c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            mpi_sys::MPI_Group,
            ::std::os::raw::c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Comm_create_group")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, group, tag, newcomm) }
    }
    #[inline]
    fn file_open<F>(
        next_f: UnsafeBox<F>,
        comm: mpi_sys::MPI_Comm,
        filename: *const ::std::os::raw::c_char,
        amode: ::std::os::raw::c_int,
        info: mpi_sys::MPI_Info,
        fh: *mut mpi_sys::MPI_File,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            *const ::std::os::raw::c_char,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Info,
            *mut mpi_sys::MPI_File,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_open")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(comm, filename, amode, info, fh) }
    }
    #[inline]
    fn file_close<F>(next_f: UnsafeBox<F>, fh: *mut mpi_sys::MPI_File) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_File) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_close")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh) }
    }
    #[inline]
    fn file_delete<F>(
        next_f: UnsafeBox<F>,
        filename: *const ::std::os::raw::c_char,
        info: mpi_sys::MPI_Info,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*const ::std::os::raw::c_char, mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_delete")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(filename, info) }
    }
    #[inline]
    fn file_set_size<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        size: mpi_sys::MPI_Offset,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Offset) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_set_size")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, size) }
    }
    #[inline]
    fn file_preallocate<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        size: mpi_sys::MPI_Offset,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Offset) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_preallocate")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, size) }
    }
    #[inline]
    fn file_get_size<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        size: *mut mpi_sys::MPI_Offset,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Offset) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_size")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, size) }
    }
    #[inline]
    fn file_get_group<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        group: *mut mpi_sys::MPI_Group,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Group) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_group")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, group) }
    }
    #[inline]
    fn file_get_amode<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        amode: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_amode")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, amode) }
    }
    #[inline]
    fn file_set_info<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        info: mpi_sys::MPI_Info,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_set_info")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, info) }
    }
    #[inline]
    fn file_get_info<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        info_used: *mut mpi_sys::MPI_Info,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_info")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, info_used) }
    }
    #[inline]
    fn file_set_view<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        disp: mpi_sys::MPI_Offset,
        etype: mpi_sys::MPI_Datatype,
        filetype: mpi_sys::MPI_Datatype,
        datarep: *const ::std::os::raw::c_char,
        info: mpi_sys::MPI_Info,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Datatype,
            *const ::std::os::raw::c_char,
            mpi_sys::MPI_Info,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_set_view")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, disp, etype, filetype, datarep, info) }
    }
    #[inline]
    fn file_get_view<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        disp: *mut mpi_sys::MPI_Offset,
        etype: *mut mpi_sys::MPI_Datatype,
        filetype: *mut mpi_sys::MPI_Datatype,
        datarep: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut mpi_sys::MPI_Offset,
            *mut mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
            *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_view")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, disp, etype, filetype, datarep) }
    }
    #[inline]
    fn file_read_at<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_at")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, status) }
    }
    #[inline]
    fn file_read_at_all<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_at_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write_at<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_at")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write_at_all<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_at_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, status) }
    }
    #[inline]
    fn file_iread_at<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_iread_at")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iwrite_at<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_iwrite_at")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, request) }
    }
    #[inline]
    fn file_read<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_read_all<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write_all<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_iread<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_iread")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iwrite<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_iwrite")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
    #[inline]
    fn file_seek<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_seek")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, whence) }
    }
    #[inline]
    fn file_get_position<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: *mut mpi_sys::MPI_Offset,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Offset) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_position")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset) }
    }
    #[inline]
    fn file_get_byte_offset<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        disp: *mut mpi_sys::MPI_Offset,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut mpi_sys::MPI_Offset,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_byte_offset")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, disp) }
    }
    #[inline]
    fn file_read_shared<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_shared")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write_shared<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_shared")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_iread_shared<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_iread_shared")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iwrite_shared<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_iwrite_shared")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
    #[inline]
    fn file_read_ordered<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_ordered")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_write_ordered<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_ordered")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, status) }
    }
    #[inline]
    fn file_seek_shared<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_seek_shared")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, whence) }
    }
    #[inline]
    fn file_get_position_shared<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: *mut mpi_sys::MPI_Offset,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Offset) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_position_shared")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset) }
    }
    #[inline]
    fn file_read_at_all_begin<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_at_all_begin")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype) }
    }
    #[inline]
    fn file_read_at_all_end<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_at_all_end")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_write_at_all_begin<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_at_all_begin")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype) }
    }
    #[inline]
    fn file_write_at_all_end<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_at_all_end")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_read_all_begin<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_all_begin")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype) }
    }
    #[inline]
    fn file_read_all_end<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_all_end")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_write_all_begin<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_all_begin")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype) }
    }
    #[inline]
    fn file_write_all_end<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_all_end")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_read_ordered_begin<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_ordered_begin")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype) }
    }
    #[inline]
    fn file_read_ordered_end<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_read_ordered_end")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_write_ordered_begin<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_ordered_begin")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype) }
    }
    #[inline]
    fn file_write_ordered_end<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            *mut mpi_sys::MPI_Status,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_write_ordered_end")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, status) }
    }
    #[inline]
    fn file_get_type_extent<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        datatype: mpi_sys::MPI_Datatype,
        extent: *mut mpi_sys::MPI_Aint,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Aint,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_type_extent")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, datatype, extent) }
    }
    #[inline]
    fn register_datarep<F>(
        next_f: UnsafeBox<F>,
        datarep: *const ::std::os::raw::c_char,
        read_conversion_fn: mpi_sys::MPI_Datarep_conversion_function,
        write_conversion_fn: mpi_sys::MPI_Datarep_conversion_function,
        dtype_file_extent_fn: mpi_sys::MPI_Datarep_extent_function,
        extra_state: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            *const ::std::os::raw::c_char,
            mpi_sys::MPI_Datarep_conversion_function,
            mpi_sys::MPI_Datarep_conversion_function,
            mpi_sys::MPI_Datarep_extent_function,
            *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("Register_datarep")
            .or_insert(0) += 1;
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
    fn file_set_atomicity<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_set_atomicity")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, flag) }
    }
    #[inline]
    fn file_get_atomicity<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_get_atomicity")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, flag) }
    }
    #[inline]
    fn file_sync<F>(next_f: UnsafeBox<F>, fh: mpi_sys::MPI_File) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_sync")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh) }
    }
    #[inline]
    fn file_iread_at_all<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_iread_at_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iwrite_at_all<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_iwrite_at_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, offset, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iread_all<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_iread_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
    #[inline]
    fn file_iwrite_all<F>(
        next_f: UnsafeBox<F>,
        fh: mpi_sys::MPI_File,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const ::std::os::raw::c_void,
            ::std::os::raw::c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> ::std::os::raw::c_int,
    {
        *MPI_FN_COUNTER_MAP
            .lock()
            .unwrap()
            .entry("File_iwrite_all")
            .or_insert(0) += 1;
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
}

pub mod tool {
    use super::install_mpi_layer;
    install_mpi_layer!(super::MyQmpiLayer);
}

use std::{convert::TryInto, sync::atomic::Ordering::Relaxed};

use self::rmpi::{MpiOp, RmpiContext};
use mpi_func_id::MPI_FUNCTION_COUNT;
use mpi_tool_layer::{RawMpiInterceptionLayer, UnsafeBox};
use qmpi_tool_creator::{install_qmpi_layer as install_mpi_layer, mpi_sys};
use rmpi::pmpi_mode as rmpi;

use mpi_func_id::MpiFunctionId as FId;

mod counter_array;
use counter_array::MPI_FN_COUNTER_MAP;

pub mod tool {
    use super::install_mpi_layer;
    install_mpi_layer!(super::MyQmpiLayer);
}

struct MyQmpiLayer;
impl RawMpiInterceptionLayer for MyQmpiLayer {
    #[inline]
    fn finalize<F>(next_f: UnsafeBox<F>) -> ::std::os::raw::c_int
    where
        F: FnOnce() -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Finalize as usize].fetch_add(1, Relaxed);

        let ctx = unsafe { RmpiContext::create_unchecked_ref() };
        let comm_world = ctx.world();

        let mut fn_counts = [0; MPI_FUNCTION_COUNT as usize];
        for fnid in 0..fn_counts.len() {
            let fnid = fnid as usize;
            fn_counts[fnid] = MPI_FN_COUNTER_MAP[fnid].load(Relaxed);
        }

        let mut all_fn_counts = [0; MPI_FUNCTION_COUNT as usize];
        comm_world
            .allreduce(&fn_counts[..], &mut all_fn_counts[..], MpiOp::Sum)
            .unwrap();

        let res = unsafe { next_f.unwrap()() };

        for fnid in 0u16.. {
            let fnid: FId = match fnid.try_into() {
                Ok(id) => id,
                Err(()) => break,
            };
            let call_count = all_fn_counts[fnid as usize];
            if call_count != 0 {
                println!("MPI_{:?}: {}", fnid, call_count);
            }
        }
        res
    }

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
        MPI_FN_COUNTER_MAP[FId::Send as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Recv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Get_count as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Bsend as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ssend as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Rsend as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Buffer_attach as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Buffer_detach as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Isend as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ibsend as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Issend as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Irsend as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Irecv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Wait as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Test as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Request_free as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Waitany as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Testany as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Waitall as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Testall as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Waitsome as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Testsome as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Iprobe as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Probe as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(source, tag, comm, status) }
    }
    #[inline]
    fn cancel<F>(next_f: UnsafeBox<F>, request: *mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Cancel as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Test_cancelled as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Send_init as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Bsend_init as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ssend_init as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Rsend_init as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Recv_init as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(buf, count, datatype, source, tag, comm, request) }
    }
    #[inline]
    fn start<F>(next_f: UnsafeBox<F>, request: *mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Start as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Startall as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Sendrecv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Sendrecv_replace as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_contiguous as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_vector as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_hvector as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_indexed as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_hindexed as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_struct as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Address as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_extent as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_size as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_lb as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_ub as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_commit as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_free as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Get_elements as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Pack as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Unpack as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Pack_size as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(incount, datatype, comm, size) }
    }
    #[inline]
    fn barrier<F>(next_f: UnsafeBox<F>, comm: mpi_sys::MPI_Comm) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Barrier as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Bcast as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Gather as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Gatherv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Scatter as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Scatterv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Allgather as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Allgatherv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Alltoall as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Alltoallv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Alltoallw as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Exscan as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Reduce as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Op_create as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(user_fn, commute, op) }
    }
    #[inline]
    fn op_free<F>(next_f: UnsafeBox<F>, op: *mut mpi_sys::MPI_Op) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Op) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Op_free as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Allreduce as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Reduce_scatter as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Scan as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_size as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_rank as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_translate_ranks as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_compare as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_group as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_union as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_intersection as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_difference as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_incl as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_excl as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_range_incl as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Group_range_excl as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(group, n, ranges, newgroup) }
    }
    #[inline]
    fn group_free<F>(next_f: UnsafeBox<F>, group: *mut mpi_sys::MPI_Group) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Group) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Group_free as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_size as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_rank as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_compare as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_dup as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_dup_with_info as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_create as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_split as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(comm, color, key, newcomm) }
    }
    #[inline]
    fn comm_free<F>(next_f: UnsafeBox<F>, comm: *mut mpi_sys::MPI_Comm) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Comm) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Comm_free as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_test_inter as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_remote_size as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_remote_group as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Intercomm_create as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Intercomm_merge as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Keyval_create as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Keyval_free as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Attr_put as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Attr_get as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Attr_delete as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Topo_test as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Cart_create as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Dims_create as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Graph_create as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Graphdims_get as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Graph_get as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Cartdim_get as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Cart_get as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Cart_rank as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Cart_coords as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Graph_neighbors_count as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Graph_neighbors as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Cart_shift as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Cart_sub as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Cart_map as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Graph_map as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Get_processor_name as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Get_version as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Get_library_version as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Errhandler_create as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Errhandler_set as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Errhandler_get as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Errhandler_free as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Error_string as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Error_class as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(errorcode, errorclass) }
    }
    #[inline]
    fn wtime<F>(next_f: UnsafeBox<F>) -> f64
    where
        F: FnOnce() -> f64,
    {
        MPI_FN_COUNTER_MAP[FId::Wtime as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()() }
    }
    #[inline]
    fn wtick<F>(next_f: UnsafeBox<F>) -> f64
    where
        F: FnOnce() -> f64,
    {
        MPI_FN_COUNTER_MAP[FId::Wtick as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Init as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(argc, argv) }
    }
    #[inline]
    fn initialized<F>(
        next_f: UnsafeBox<F>,
        flag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Initialized as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Abort as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(comm, errorcode) }
    }
    #[inline]
    fn pcontrol<F>(next_f: UnsafeBox<F>, level: ::std::os::raw::c_int) -> ::std::os::raw::c_int
    where
        F: FnOnce(::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Pcontrol as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Close_port as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_accept as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_connect as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_disconnect as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_get_parent as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_join as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Lookup_name as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Open_port as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Publish_name as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Unpublish_name as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_set_info as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_get_info as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Accumulate as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Get as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Put as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_complete as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_create as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_fence as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(assert, win) }
    }
    #[inline]
    fn win_free<F>(next_f: UnsafeBox<F>, win: *mut mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Win_free as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_get_group as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_lock as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_post as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_start as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_test as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_unlock as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(rank, win) }
    }
    #[inline]
    fn win_wait<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Win_wait as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_allocate as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_allocate_shared as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_shared_query as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_create_dynamic as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_attach as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_detach as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_get_info as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_set_info as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Get_accumulate as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Fetch_and_op as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Compare_and_swap as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Rput as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Rget as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Raccumulate as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Rget_accumulate as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_lock_all as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(assert, win) }
    }
    #[inline]
    fn win_unlock_all<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Win_unlock_all as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_flush as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(rank, win) }
    }
    #[inline]
    fn win_flush_all<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Win_flush_all as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_flush_local as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(rank, win) }
    }
    #[inline]
    fn win_flush_local_all<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Win_flush_local_all as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(win) }
    }
    #[inline]
    fn win_sync<F>(next_f: UnsafeBox<F>, win: mpi_sys::MPI_Win) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Win_sync as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Add_error_class as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Add_error_code as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Add_error_string as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_call_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_create_keyval as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_delete_attr as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_free_keyval as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_get_attr as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_get_name as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_set_attr as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_set_name as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_call_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Grequest_complete as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Grequest_start as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Init_thread as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Is_thread_main as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Query_thread as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Status_set_cancelled as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Status_set_elements as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_keyval as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_delete_attr as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_dup as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_free_keyval as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_get_attr as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_get_contents as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_get_envelope as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_get_name as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_set_attr as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_set_name as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_match_size as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_call_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_create_keyval as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_delete_attr as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_free_keyval as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_get_attr as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_get_name as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_set_attr as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_set_name as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Alloc_mem as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_create_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_get_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_set_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_create_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_set_errhandler as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(file, errhandler) }
    }
    #[inline]
    fn finalized<F>(next_f: UnsafeBox<F>, flag: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Finalized as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(flag) }
    }
    #[inline]
    fn free_mem<F>(next_f: UnsafeBox<F>, base: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Free_mem as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Get_address as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(location, address) }
    }
    #[inline]
    fn info_create<F>(next_f: UnsafeBox<F>, info: *mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Info_create as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Info_delete as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Info_dup as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(info, newinfo) }
    }
    #[inline]
    fn info_free<F>(next_f: UnsafeBox<F>, info: *mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Info) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::Info_free as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Info_get as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Info_get_nkeys as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Info_get_nthkey as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Info_get_valuelen as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Info_set as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Pack_external as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Pack_external_size as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Request_get_status as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_darray as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_hindexed as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_hvector as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_indexed_block as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_hindexed_block as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_resized as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_struct as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_subarray as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_get_extent as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_get_true_extent as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Unpack_external as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_create_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_get_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Win_set_errhandler as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_f90_integer as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_f90_real as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_create_f90_complex as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Reduce_local as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Op_commutative as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Reduce_scatter_block as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Dist_graph_create_adjacent as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Dist_graph_create as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Dist_graph_neighbors_count as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Dist_graph_neighbors as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Improbe as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Imrecv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Mprobe as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Mrecv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_idup as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ibarrier as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ibcast as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Igather as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Igatherv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Iscatter as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Iscatterv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Iallgather as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Iallgatherv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ialltoall as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ialltoallv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ialltoallw as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ireduce as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Iallreduce as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ireduce_scatter as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ireduce_scatter_block as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Iscan as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Iexscan as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ineighbor_allgather as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ineighbor_allgatherv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ineighbor_alltoall as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ineighbor_alltoallv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Ineighbor_alltoallw as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Neighbor_allgather as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Neighbor_allgatherv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Neighbor_alltoall as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Neighbor_alltoallv as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Neighbor_alltoallw as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_split_type as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Get_elements_x as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Status_set_elements_x as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_get_extent_x as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_get_true_extent_x as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Type_size_x as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Comm_create_group as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_open as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(comm, filename, amode, info, fh) }
    }
    #[inline]
    fn file_close<F>(next_f: UnsafeBox<F>, fh: *mut mpi_sys::MPI_File) -> ::std::os::raw::c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_File) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::File_close as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_delete as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_set_size as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_preallocate as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_size as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_group as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_amode as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_set_info as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_info as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_set_view as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_view as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_at as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_at_all as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_at as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_at_all as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_iread_at as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_iwrite_at as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_all as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_all as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_iread as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_iwrite as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_seek as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_position as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_byte_offset as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_shared as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_shared as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_iread_shared as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_iwrite_shared as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_ordered as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_ordered as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_seek_shared as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_position_shared as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_at_all_begin as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_at_all_end as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_at_all_begin as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_at_all_end as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_all_begin as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_all_end as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_all_begin as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_all_end as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_ordered_begin as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_read_ordered_end as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_ordered_begin as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_write_ordered_end as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_type_extent as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::Register_datarep as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_set_atomicity as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_get_atomicity as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(fh, flag) }
    }
    #[inline]
    fn file_sync<F>(next_f: UnsafeBox<F>, fh: mpi_sys::MPI_File) -> ::std::os::raw::c_int
    where
        F: FnOnce(mpi_sys::MPI_File) -> ::std::os::raw::c_int,
    {
        MPI_FN_COUNTER_MAP[FId::File_sync as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_iread_at_all as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_iwrite_at_all as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_iread_all as usize].fetch_add(1, Relaxed);
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
        MPI_FN_COUNTER_MAP[FId::File_iwrite_all as usize].fetch_add(1, Relaxed);
        unsafe { next_f.unwrap()(fh, buf, count, datatype, request) }
    }
}

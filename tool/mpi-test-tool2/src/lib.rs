use std::{
    os::raw::c_void,
    ptr,
    sync::atomic::{AtomicU32, Ordering::*},
};

use mpi_tool_layer::MpiInterceptionLayer;
use pmpi_tool_creator::mpi_sys::*;
use rmpi::{Buffer, Process, RmpiResult, Status, Tag};

static BYTE_SEND_COUNTER: AtomicU32 = AtomicU32::new(0);
static BYTE_RECV_COUNTER: AtomicU32 = AtomicU32::new(0);

struct MyQmpiLayer;
impl MpiInterceptionLayer for MyQmpiLayer {
    fn send<F, Buf: ?Sized>(next_f: F, buf: &Buf, dest: Process, tag: Tag) -> RmpiResult
    where
        F: FnOnce(&Buf, Process, Tag) -> RmpiResult,
        Buf: Buffer,
    {
        BYTE_SEND_COUNTER.fetch_add(buf.as_bytes().len() as _, Relaxed);
        eprintln!("[sent {} bytes]", buf.as_bytes().len());
        next_f(buf, dest, tag)
    }
    fn recv<F, Buf: ?Sized>(next_f: F, buf: &mut Buf, src: Process, tag: Tag) -> RmpiResult<Status>
    where
        F: FnOnce(&mut Buf, Process, Tag) -> RmpiResult<Status>,
        Buf: Buffer,
    {
        BYTE_RECV_COUNTER.fetch_add(buf.as_bytes().len() as _, Relaxed);
        eprintln!("[received {} bytes]", buf.as_bytes().len());
        next_f(buf, src, tag)
    }

    fn finalize<F>(next_f: F) -> RmpiResult
    where
        F: FnOnce() -> RmpiResult,
    {
        let mut rank = 0;
        unsafe { MPI_Comm_rank(MPI_COMM_WORLD, &mut rank) };

        const COUNTER_LEN: usize = 2;
        let byte_count_arr: [u32; COUNTER_LEN] = [
            BYTE_SEND_COUNTER.load(Relaxed),
            BYTE_RECV_COUNTER.load(Relaxed),
        ];

        let mut count_sum = [0u32; COUNTER_LEN];
        if rank == 0 {
            unsafe {
                MPI_Reduce(
                    &byte_count_arr[0] as *const u32 as *const c_void,
                    &mut count_sum[0] as *mut u32 as *mut c_void,
                    byte_count_arr.len() as _,
                    MPI_UINT32_T,
                    MPI_SUM,
                    0,
                    MPI_COMM_WORLD,
                );
            }
            eprintln!("[sent {} bytes total]", count_sum[0]);
            eprintln!("[received {} bytes total]", count_sum[1]);
        } else {
            unsafe {
                MPI_Reduce(
                    &byte_count_arr[0] as *const u32 as *const c_void,
                    ptr::null_mut() as *mut u32 as *mut c_void,
                    byte_count_arr.len() as _,
                    MPI_UINT32_T,
                    MPI_SUM,
                    0,
                    MPI_COMM_WORLD,
                );
            }
        }

        next_f()
    }
}

pub mod tool {
    pmpi_tool_creator::install_pmpi_layer!(super::MyQmpiLayer);
}

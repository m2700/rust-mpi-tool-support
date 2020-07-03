use std::{
    os::raw::{c_char, c_int, c_void},
    ptr,
    sync::atomic::{AtomicU32, Ordering::*},
};

use mpi_sys::{MPI_Comm_rank, MPI_SUM, MPI_UINT32_T};
use mpi_tool_layer::{RawMpiInterceptionLayer, UnsafeBox};
use pmpi_tool_creator::mpi_sys;

static INIT_CALL_COUNTER: AtomicU32 = AtomicU32::new(0);
static PCONTROL_CALL_COUNTER: AtomicU32 = AtomicU32::new(0);
static COMM_SIZE_CALL_COUNTER: AtomicU32 = AtomicU32::new(0);
static COMM_RANK_CALL_COUNTER: AtomicU32 = AtomicU32::new(0);
static BCAST_CALL_COUNTER: AtomicU32 = AtomicU32::new(0);
static BARRIER_CALL_COUNTER: AtomicU32 = AtomicU32::new(0);
static FINALIZE_CALL_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Default)]
struct MyQmpiLayer;
impl RawMpiInterceptionLayer for MyQmpiLayer {
    fn init<F>(next_f: UnsafeBox<F>, argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int
    where
        F: FnOnce(*mut c_int, *mut *mut *mut c_char) -> c_int,
    {
        let ccnt = INIT_CALL_COUNTER.fetch_add(1, Relaxed) + 1;
        println!("[called init(..) count: {}]", ccnt);
        unsafe { next_f.unwrap()(argc, argv) }
    }
    fn pcontrol<F>(next_f: UnsafeBox<F>, level: c_int) -> c_int
    where
        F: FnOnce(c_int) -> c_int,
    {
        let ccnt = PCONTROL_CALL_COUNTER.fetch_add(1, Relaxed) + 1;
        println!("[called pcontrol({}) count: {}]", level, ccnt);
        println!("[call pcontrol({}) 'incremented']", level + 1);
        unsafe { next_f.unwrap()(level + 1) }
    }
    fn comm_size<F>(next_f: UnsafeBox<F>, comm: mpi_sys::MPI_Comm, size: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        let ccnt = COMM_SIZE_CALL_COUNTER.fetch_add(1, Relaxed) + 1;
        let mut rank = 0;
        unsafe { MPI_Comm_rank(comm, &mut rank) };
        println!("[called comm_size(..) on rank: {}, count: {}]", rank, ccnt);
        unsafe { next_f.unwrap()(comm, size) }
    }
    fn comm_rank<F>(next_f: UnsafeBox<F>, comm: mpi_sys::MPI_Comm, rank: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        let ccnt = COMM_RANK_CALL_COUNTER.fetch_add(1, Relaxed) + 1;

        let res = unsafe { next_f.unwrap()(comm, rank) };
        println!(
            "[called comm_rank(..) -> {}, count: {}]",
            unsafe { *rank },
            ccnt
        );
        res
    }
    fn bcast<F>(
        next_f: UnsafeBox<F>,
        buffer: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*mut c_void, c_int, mpi_sys::MPI_Datatype, c_int, mpi_sys::MPI_Comm) -> c_int,
    {
        let ccnt = BCAST_CALL_COUNTER.fetch_add(1, Relaxed) + 1;
        let mut rank = 0;
        unsafe { MPI_Comm_rank(comm, &mut rank) };
        println!("[called bcast(..) on rank: {}, count: {}]", rank, ccnt);
        unsafe { next_f.unwrap()(buffer, count, datatype, root, comm) }
    }
    fn barrier<F>(next_f: UnsafeBox<F>, comm: mpi_sys::MPI_Comm) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm) -> c_int,
    {
        let ccnt = BARRIER_CALL_COUNTER.fetch_add(1, Relaxed) + 1;
        let mut rank = 0;
        unsafe { MPI_Comm_rank(comm, &mut rank) };
        println!("[called barrier(..) on rank: {}, count: {}]", rank, ccnt);
        unsafe { next_f.unwrap()(comm) }
    }
    fn finalize<F>(next_f: UnsafeBox<F>) -> c_int
    where
        F: FnOnce() -> c_int,
    {
        let mut rank = 0;
        unsafe { MPI_Comm_rank(mpi_sys::MPI_COMM_WORLD, &mut rank) };
        let finalize_count = FINALIZE_CALL_COUNTER.fetch_add(1, SeqCst) + 1;
        println!(
            "[called finalize(..), on rank: {}, count: {}]",
            rank, finalize_count
        );

        let init_count = INIT_CALL_COUNTER.load(SeqCst);
        let pcontrol_count = PCONTROL_CALL_COUNTER.load(SeqCst);
        let comm_size_count = COMM_SIZE_CALL_COUNTER.load(SeqCst);
        let comm_rank_count = COMM_RANK_CALL_COUNTER.load(SeqCst);
        let bcast_count = BCAST_CALL_COUNTER.load(SeqCst);
        let barrier_count = BARRIER_CALL_COUNTER.load(SeqCst);

        const COUNTS_ARR_LEN: usize = 6;
        let counts: [u32; COUNTS_ARR_LEN] = [
            init_count,
            pcontrol_count,
            comm_size_count,
            comm_rank_count,
            bcast_count,
            barrier_count,
        ];
        let mut count_sums = [0u32; COUNTS_ARR_LEN];
        if rank == 0 {
            unsafe {
                mpi_sys::MPI_Reduce(
                    &counts[0] as *const u32 as *const c_void,
                    &mut count_sums[0] as *mut u32 as *mut c_void,
                    COUNTS_ARR_LEN as _,
                    MPI_UINT32_T,
                    MPI_SUM,
                    0,
                    mpi_sys::MPI_COMM_WORLD,
                );
            }
        } else {
            unsafe {
                mpi_sys::MPI_Reduce(
                    &counts[0] as *const u32 as *const c_void,
                    ptr::null_mut() as *mut u32 as *mut c_void,
                    COUNTS_ARR_LEN as _,
                    MPI_UINT32_T,
                    MPI_SUM,
                    0,
                    mpi_sys::MPI_COMM_WORLD,
                );
            }
        }

        unsafe { next_f.unwrap()() }
    }
}

pub mod tool {
    pmpi_tool_creator::install_pmpi_layer!(super::MyQmpiLayer);
}

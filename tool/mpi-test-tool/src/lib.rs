use std::os::raw::{c_char, c_int, c_void};

use mpi_tool_layer::{RawMpiInterceptionLayer, UnsafeBox};
use pmpi_tool_creator::mpi_sys;

#[derive(Default)]
struct MyQmpiLayer;
impl RawMpiInterceptionLayer for MyQmpiLayer {
    fn init<F>(&self, next_f: UnsafeBox<F>, argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int
    where
        F: FnOnce(*mut c_int, *mut *mut *mut c_char) -> c_int,
    {
        println!("[called init(..)]");
        unsafe { next_f.unwrap()(argc, argv) }
    }
    fn pcontrol<F>(&self, next_f: UnsafeBox<F>, level: c_int) -> c_int
    where
        F: FnOnce(c_int) -> c_int,
    {
        println!("[called pcontrol({})]", level);
        println!("[call pcontrol({}) 'incremented']", level + 1);
        unsafe { next_f.unwrap()(level + 1) }
    }
    fn comm_size<F>(&self, next_f: UnsafeBox<F>, comm: mpi_sys::MPI_Comm, size: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        println!("[called comm_size(..)]");
        unsafe { next_f.unwrap()(comm, size) }
    }
    fn comm_rank<F>(&self, next_f: UnsafeBox<F>, comm: mpi_sys::MPI_Comm, rank: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        println!("[called comm_rank(..)]");
        unsafe { next_f.unwrap()(comm, rank) }
    }
    fn bcast<F>(
        &self,
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
        println!("[called bcast(..)]");
        unsafe { next_f.unwrap()(buffer, count, datatype, root, comm) }
    }
    fn barrier<F>(&self, next_f: UnsafeBox<F>, comm: mpi_sys::MPI_Comm) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm) -> c_int,
    {
        println!("[called barrier(..)]");
        unsafe { next_f.unwrap()(comm) }
    }
    fn finalize<F>(&self, next_f: UnsafeBox<F>) -> c_int
    where
        F: FnOnce() -> c_int,
    {
        println!("[called finalize(..)]");
        unsafe { next_f.unwrap()() }
    }
}

#[cfg(feature = "qmpi_mode")]
mpi_tool_creator::install_qmpi_layer!(MyQmpiLayer);
#[cfg(feature = "pmpi_mode")]
mpi_tool_creator::install_pmpi_layer!(MyQmpiLayer);

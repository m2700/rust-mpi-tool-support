use std::os::raw::{c_char, c_int, c_void};

use qmpi_tool_creator::{install_layer, mpi_sys, QmpiLayer};

#[derive(Default)]
struct MyQmpiLayer;
impl QmpiLayer for MyQmpiLayer {
    fn init<F>(&self, next_f: F, argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int
    where
        F: FnOnce(*mut c_int, *mut *mut *mut c_char) -> c_int,
    {
        println!("[called init(..)]");
        next_f(argc, argv)
    }
    fn pcontrol<F>(&self, next_f: F, level: c_int) -> c_int
    where
        F: FnOnce(c_int) -> c_int,
    {
        println!("[called pcontrol({})]", level);
        println!("[call pcontrol({}) 'incremented']", level + 1);
        next_f(level + 1)
    }
    fn comm_size<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, size: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        println!("[called comm_size(..)]");
        next_f(comm, size)
    }
    fn comm_rank<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, rank: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        println!("[called comm_rank(..)]");
        next_f(comm, rank)
    }
    fn bcast<F>(
        &self,
        next_f: F,
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
        next_f(buffer, count, datatype, root, comm)
    }
    fn barrier<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm) -> c_int,
    {
        println!("[called barrier(..)]");
        next_f(comm)
    }
    fn finalize<F>(&self, next_f: F) -> c_int
    where
        F: FnOnce() -> c_int,
    {
        println!("[called finalize(..)]");
        next_f()
    }
}

install_layer!(MyQmpiLayer);

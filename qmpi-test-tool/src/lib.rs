use std::os::raw::{c_char, c_int, c_void};

use qmpi_tool_creator::{install_layer, mpi_sys, QmpiLayer};

struct MyQmpiLayer;
impl QmpiLayer for MyQmpiLayer {
    fn pre_init(_argc: *mut c_int, _argv: *mut *mut *mut c_char) {
        println!("pre_init");
    }
    fn post_init(output: c_int, _argc: *mut c_int, _argv: *mut *mut *mut c_char) {
        println!("post_init: {}", output);
    }

    fn pre_comm_size(_comm: mpi_sys::MPI_Comm, _size: *mut c_int) {
        println!("pre_comm_size");
    }
    fn post_comm_size(output: c_int, _comm: mpi_sys::MPI_Comm, _size: *mut c_int) {
        println!("post_comm_size: {}", output);
    }

    fn pre_comm_rank(_comm: mpi_sys::MPI_Comm, _rank: *mut c_int) {
        println!("pre_comm_rank");
    }
    fn post_comm_rank(output: c_int, _comm: mpi_sys::MPI_Comm, _rank: *mut c_int) {
        println!("post_comm_rank: {}", output);
    }

    fn pre_bcast(
        _buffer: *mut c_void,
        _count: c_int,
        _datatype: mpi_sys::MPI_Datatype,
        _root: c_int,
        _comm: mpi_sys::MPI_Comm,
    ) {
        println!("pre_bcast");
    }
    fn post_bcast(
        output: c_int,
        _buffer: *mut c_void,
        _count: c_int,
        _datatype: mpi_sys::MPI_Datatype,
        _root: c_int,
        _comm: mpi_sys::MPI_Comm,
    ) {
        println!("post_bcast: {}", output);
    }

    fn pre_barrier(_comm: mpi_sys::MPI_Comm) {
        println!("pre_barrier");
    }
    fn post_barrier(output: c_int, _comm: mpi_sys::MPI_Comm) {
        println!("post_barrier: {}", output);
    }

    fn pre_finalize() {
        println!("pre_finalize");
    }
    fn post_finalize(output: c_int) {
        println!("post_finalize: {}", output);
    }
}

install_layer!(MyQmpiLayer);

use std::os::raw::{c_char, c_int, c_void};

use qmpi_tool_creator::{install_layer, mpi_sys, QmpiLayer};

#[derive(Default)]
struct MyQmpiLayer;
impl QmpiLayer for MyQmpiLayer {
    fn pre_init(&mut self, _argc: *mut c_int, _argv: *mut *mut *mut c_char) {
        println!("pre_init");
    }
    fn post_init(&mut self, output: c_int, _argc: *mut c_int, _argv: *mut *mut *mut c_char) {
        println!("post_init: {}", output);
    }

    fn pre_pcontrol(&mut self, level: c_int) {
        println!("pre_pcontrol({})", level);
    }
    fn post_pcontrol(&mut self, output: c_int, level: c_int) {
        println!("post_pcontrol({}): {}", level, output);
    }

    fn pre_comm_size(&mut self, _comm: mpi_sys::MPI_Comm, _size: *mut c_int) {
        println!("pre_comm_size");
    }
    fn post_comm_size(&mut self, output: c_int, _comm: mpi_sys::MPI_Comm, _size: *mut c_int) {
        println!("post_comm_size: {}", output);
    }

    fn pre_comm_rank(&mut self, _comm: mpi_sys::MPI_Comm, _rank: *mut c_int) {
        println!("pre_comm_rank");
    }
    fn post_comm_rank(&mut self, output: c_int, _comm: mpi_sys::MPI_Comm, _rank: *mut c_int) {
        println!("post_comm_rank: {}", output);
    }

    fn pre_bcast(
        &mut self,
        _buffer: *mut c_void,
        _count: c_int,
        _datatype: mpi_sys::MPI_Datatype,
        _root: c_int,
        _comm: mpi_sys::MPI_Comm,
    ) {
        println!("pre_bcast");
    }
    fn post_bcast(
        &mut self,
        output: c_int,
        _buffer: *mut c_void,
        _count: c_int,
        _datatype: mpi_sys::MPI_Datatype,
        _root: c_int,
        _comm: mpi_sys::MPI_Comm,
    ) {
        println!("post_bcast: {}", output);
    }

    fn pre_barrier(&mut self, _comm: mpi_sys::MPI_Comm) {
        println!("pre_barrier");
    }
    fn post_barrier(&mut self, output: c_int, _comm: mpi_sys::MPI_Comm) {
        println!("post_barrier: {}", output);
    }

    fn pre_finalize(&mut self) {
        println!("pre_finalize");
    }
    fn post_finalize(&mut self, output: c_int) {
        println!("post_finalize: {}", output);
    }
}

install_layer!(MyQmpiLayer);

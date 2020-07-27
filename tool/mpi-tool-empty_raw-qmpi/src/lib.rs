use std::os::raw::c_int;

use mpi_tool_layer::{RawMpiInterceptionLayer, UnsafeBox};

struct MyQmpiLayer;
impl RawMpiInterceptionLayer for MyQmpiLayer {
    fn finalize<F>(next_f: UnsafeBox<F>) -> c_int
    where
        F: FnOnce() -> c_int,
    {
        println!("called finalize (raw qmpi)");
        (unsafe { next_f.unwrap() })()
    }
}

pub mod tool {
    qmpi_tool_creator::install_qmpi_layer!(super::MyQmpiLayer);
}

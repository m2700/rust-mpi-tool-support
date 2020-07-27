use std::os::raw::c_int;

use mpi_tool_layer::{RawMpiInterceptionLayer, UnsafeBox};

struct MyPmpiLayer;
impl RawMpiInterceptionLayer for MyPmpiLayer {
    fn finalize<F>(next_f: UnsafeBox<F>) -> c_int
    where
        F: FnOnce() -> c_int,
    {
        println!("called finalize (raw pmpi)");
        (unsafe { next_f.unwrap() })()
    }
}

pub mod tool {
    pmpi_tool_creator::install_pmpi_layer!(super::MyPmpiLayer);
}

use std::{env, os::raw::c_int};

use mpi_tool_layer::{RawMpiInterceptionLayer, UnsafeBox};

struct MyPmpiLayer;
impl RawMpiInterceptionLayer for MyPmpiLayer {
    fn finalize<F>(next_f: UnsafeBox<F>) -> c_int
    where
        F: FnOnce() -> c_int,
    {
        let fin_dbg_cnf = env::var("FINALIZE_DEBUG_CONFIRM");
        if fin_dbg_cnf.is_err() || fin_dbg_cnf.as_ref().map(|s| &**s) == Ok("1") {
            println!("called finalize (raw pmpi)");
        }
        (unsafe { next_f.unwrap() })()
    }
}

pub mod tool {
    pmpi_tool_creator::install_pmpi_layer!(super::MyPmpiLayer);
}

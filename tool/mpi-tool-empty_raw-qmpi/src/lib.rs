use std::{env, os::raw::c_int};

use mpi_tool_layer::{RawMpiInterceptionLayer, UnsafeBox};

struct MyQmpiLayer;
impl RawMpiInterceptionLayer for MyQmpiLayer {
    fn finalize<F>(next_f: UnsafeBox<F>) -> c_int
    where
        F: FnOnce() -> c_int,
    {
        let fin_dbg_cnf = env::var("FINALIZE_DEBUG_CONFIRM");
        if fin_dbg_cnf.is_err() || fin_dbg_cnf.as_ref().map(|s| &**s) == Ok("1") {
            println!("called finalize (raw qmpi)");
        }
        (unsafe { next_f.unwrap() })()
    }
}

pub mod tool {
    qmpi_tool_creator::install_qmpi_layer!(super::MyQmpiLayer);
}

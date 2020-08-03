use std::env;

use mpi_tool_layer::MpiInterceptionLayer;
use rmpi::pmpi_mode as rmpi;

struct MyQmpiLayer;
impl MpiInterceptionLayer for MyQmpiLayer {
    fn finalize<F>(next_f: F) -> rmpi::RmpiResult
    where
        F: FnOnce() -> rmpi::RmpiResult,
    {
        let fin_dbg_cnf = env::var("FINALIZE_DEBUG_CONFIRM");
        if fin_dbg_cnf.is_err() || fin_dbg_cnf.as_ref().map(|s| &**s) == Ok("1") {
            println!("called finalize (high level qmpi)");
        }
        next_f()
    }
}

pub mod tool {
    qmpi_tool_creator::install_qmpi_layer!(super::MyQmpiLayer);
}

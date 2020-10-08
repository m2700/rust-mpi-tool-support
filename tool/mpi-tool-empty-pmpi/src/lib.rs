use std::env;

use mpi_tool_layer::{rmpi, MpiInterceptionLayer};

struct MyPmpiLayer;
impl MpiInterceptionLayer for MyPmpiLayer {
    fn finalize<F>(next_f: F, rmpi_ctx: rmpi::RmpiContext) -> rmpi::RmpiResult
    where
        F: FnOnce(rmpi::RmpiContext) -> rmpi::RmpiResult,
    {
        let fin_dbg_cnf = env::var("FINALIZE_DEBUG_CONFIRM");
        if fin_dbg_cnf.is_err() || fin_dbg_cnf.as_ref().map(|s| &**s) == Ok("1") {
            println!("called finalize (high level pmpi)");
        }
        next_f(rmpi_ctx)
    }
}

pub mod tool {
    pmpi_tool_creator::install_pmpi_layer!(super::MyPmpiLayer);
}

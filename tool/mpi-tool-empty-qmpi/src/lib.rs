use mpi_tool_layer::MpiInterceptionLayer;
use rmpi::pmpi_mode as rmpi;

struct MyQmpiLayer;
impl MpiInterceptionLayer for MyQmpiLayer {
    fn finalize<F>(next_f: F) -> rmpi::RmpiResult
    where
        F: FnOnce() -> rmpi::RmpiResult,
    {
        println!("called finalize (high level qmpi)");
        next_f()
    }
}

pub mod tool {
    qmpi_tool_creator::install_qmpi_layer!(super::MyQmpiLayer);
}

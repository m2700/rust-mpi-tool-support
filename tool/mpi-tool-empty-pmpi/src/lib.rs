use mpi_tool_layer::MpiInterceptionLayer;
use rmpi::pmpi_mode as rmpi;

struct MyPmpiLayer;
impl MpiInterceptionLayer for MyPmpiLayer {
    fn finalize<F>(next_f:F)->rmpi::RmpiResult where F:FnOnce()->rmpi::RmpiResult{
        println!("called finalize (high level pmpi)");
        next_f()
    }
}

pub mod tool {
    pmpi_tool_creator::install_pmpi_layer!(super::MyPmpiLayer);
}

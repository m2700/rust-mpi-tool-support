#[cfg(feature = "qmpi_mode")]
pub use qmpi_sys;
#[cfg(all(feature = "qmpi_mode", not(feature = "pmpi_mode")))]
pub use qmpi_sys::mpi_sys::pmpi as mpi_sys;

#[cfg(feature = "pmpi_mode")]
pub use mpi_sys::pmpi as mpi_sys;

mod layer;
#[cfg(feature = "pmpi_mode")]
mod pmpi_layer;
#[cfg(feature = "qmpi_mode")]
mod qmpi_layer;

pub use layer::MpiInterceptionLayer;

#[cfg(feature = "export_mpi")]
pub use mpi_sys;

mod c_qmpi {
    #![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
    include!(env!("QMPI_BINDGEN_BINDINGS"));
}

pub use c_qmpi::{_MPI_funcs, mpi_func, vector, NUM_MPI_FUNCS};

#[cfg(feature = "link_qmpi")]
pub use c_qmpi::{vector_get, QMPI_Table_query};

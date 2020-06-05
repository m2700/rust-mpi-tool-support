mod c_qmpi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(env!("QMPI_BINDGEN_BINDINGS"));
}

pub use c_qmpi::{_MPI_funcs, mpi_func, vector, vector_get, QMPI_Table_query, NUM_MPI_FUNCS};

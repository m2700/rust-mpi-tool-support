use std::os::raw::c_int;

local_mod!(
    use mpi_sys::MPI_SUCCESS;
);

#[derive(Debug, PartialEq, Eq)]
pub struct Error(c_int);
impl From<Error> for c_int {
    #[inline]
    fn from(e: Error) -> Self {
        e.0
    }
}
impl Error {
    #[inline]
    pub fn from_mpi_res(mpi_res: c_int) -> Result<(), Self> {
        if mpi_res == MPI_SUCCESS {
            Ok(())
        } else {
            Err(Error(mpi_res))
        }
    }
    #[inline]
    pub fn result_into_mpi_res(res: Result<(), Self>) -> c_int {
        match res {
            Ok(()) => MPI_SUCCESS,
            Err(e) => e.0,
        }
    }
}

pub type RmpiResult<T = ()> = Result<T, Error>;

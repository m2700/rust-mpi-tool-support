use std::os::raw::c_int;

use mpi_sys::MPI_Comm;

use crate::Process;

pub struct Communicator(pub(crate) MPI_Comm);
impl Communicator {
    #[inline]
    pub unsafe fn from_raw(raw: MPI_Comm) -> Self {
        Self(raw)
    }
    #[inline]
    pub fn as_raw(&self) -> MPI_Comm {
        self.0
    }
    #[inline]
    pub fn get_process(&self, rank: c_int) -> Process {
        Process {
            communicator: self,
            rank,
        }
    }
}

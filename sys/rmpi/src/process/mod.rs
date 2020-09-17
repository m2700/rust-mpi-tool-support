use std::os::raw::c_int;

local_mod!(
    use crate::Communicator;
);

mod bcast;
mod gather;
mod irecv;
mod isend;
mod recv;
mod reduce;
mod scatter;
mod send;
mod sendrecv;

#[derive(Clone, Copy)]
pub struct Process<'c> {
    pub(crate) communicator: &'c Communicator,
    pub(crate) rank: c_int,
}
impl<'c> Process<'c> {
    #[inline]
    pub fn rank(&self) -> c_int {
        self.rank
    }
    #[inline]
    pub fn communicator(&self) -> &'c Communicator {
        self.communicator
    }
}

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
mod send;
mod sendrecv;

#[derive(Clone, Copy)]
pub struct Process<'c> {
    pub(crate) communicator: &'c Communicator,
    pub(crate) rank: c_int,
}

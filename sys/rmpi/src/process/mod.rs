use std::os::raw::c_int;

use crate::Communicator;

mod irecv;
mod isend;
mod recv;
mod send;mod bcast;

pub struct Process<'c> {
    pub(crate) communicator: &'c Communicator,
    pub(crate) rank: c_int,
}

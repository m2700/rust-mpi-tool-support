use crate::RawMpiInterceptionLayer;

pub trait MpiInterceptionLayer {}

impl<T> RawMpiInterceptionLayer for T where T: MpiInterceptionLayer {}

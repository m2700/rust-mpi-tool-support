use std::{ops::Deref, os::raw::*};

local_mod!(
    use mpi_sys::*;
);

#[derive(Clone, Copy, Default)]
pub struct Tag(c_int);
impl Tag {
    #[inline]
    pub const fn new(tag: c_int) -> Self {
        Self(tag)
    }
    #[inline]
    pub const fn any() -> Self {
        Self(MPI_ANY_TAG)
    }
}
impl Deref for Tag {
    type Target = c_int;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<c_int> for Tag {
    fn from(tag: c_int) -> Self {
        Self(tag)
    }
}

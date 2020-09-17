use std::{ops::Deref, os::raw::*};

local_mod!(
    use mpi_sys::*;
);

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
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
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<c_int> for Tag {
    #[inline]
    fn from(tag: c_int) -> Self {
        Self(tag)
    }
}
impl PartialEq<c_int> for Tag {
    #[inline]
    fn eq(&self, other: &c_int) -> bool {
        self.0 == *other
    }
}
impl PartialEq<Tag> for c_int {
    #[inline]
    fn eq(&self, other: &Tag) -> bool {
        *self == other.0
    }
}

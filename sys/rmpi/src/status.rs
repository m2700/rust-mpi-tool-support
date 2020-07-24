use std::{mem::transmute, os::raw::c_int, slice};

local_mod!(
    use mpi_sys::*;
    use crate::{Error, MpiDatatype, RmpiResult};
);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Status(MPI_Status);
impl Status {
    #[inline]
    pub const fn from_raw(raw: MPI_Status) -> Self {
        Self(raw)
    }
    #[inline]
    pub const fn into_raw(self) -> MPI_Status {
        self.0
    }

    #[inline]
    pub fn from_raw_slice<'a>(raw: *const MPI_Status, len: usize) -> &'a [Self] {
        unsafe { transmute(slice::from_raw_parts(raw, len)) }
    }
    #[inline]
    pub fn from_raw_slice_mut<'a>(raw: *mut MPI_Status, len: usize) -> &'a mut [Self] {
        unsafe { transmute(slice::from_raw_parts_mut(raw, len)) }
    }
    #[inline]
    pub fn into_raw_slice(this: &[Self]) -> &[MPI_Status] {
        unsafe { transmute(this) }
    }
    #[inline]
    pub fn into_raw_slice_mut(this: &mut [Self]) -> &mut [MPI_Status] {
        unsafe { transmute(this) }
    }

    tool_mode_item! {
        #[inline]
        pub unsafe fn get_count_with<F, Datatype>(&self, get_count: F) -> RmpiResult<c_int>
            where
                F: FnOnce(*const MPI_Status, MPI_Datatype, *mut c_int) -> c_int,
                Datatype: MpiDatatype
        {
            let mut count = 0;
            Error::from_mpi_res(
                get_count(&self.into_raw(), Datatype::datatype(), &mut count)
            ).map(|()|count)
        }
    }
    #[inline]
    pub fn get_count<Datatype>(&self) -> RmpiResult<c_int>
    where
        Datatype: MpiDatatype,
    {
        unsafe {
            self.get_count_with::<_, Datatype>(|status, datatype, count| {
                MPI_Get_count(status, datatype, count)
            })
        }
    }
}

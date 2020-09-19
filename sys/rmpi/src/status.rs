use std::{
    mem::{transmute, MaybeUninit},
    os::raw::c_int,
    slice,
};

local_mod!(
    use mpi_sys::*;
    use crate::{Error, datatype::RawDatatype, RmpiResult};
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
    pub unsafe fn from_raw_slice<'a>(raw: *const MPI_Status, len: usize) -> &'a [Self] {
        transmute(slice::from_raw_parts(raw, len))
    }
    #[inline]
    pub unsafe fn from_raw_slice_mut<'a>(raw: *mut MPI_Status, len: usize) -> &'a mut [Self] {
        transmute(slice::from_raw_parts_mut(raw, len))
    }
    #[inline]
    pub unsafe fn from_raw_slice_maybe_uninit<'a>(
        raw: *const MPI_Status,
        len: usize,
    ) -> &'a [MaybeUninit<Self>] {
        transmute(slice::from_raw_parts(raw, len))
    }
    #[inline]
    pub unsafe fn from_raw_slice_mut_maybe_uninit<'a>(
        raw: *mut MPI_Status,
        len: usize,
    ) -> &'a mut [MaybeUninit<Self>] {
        transmute(slice::from_raw_parts_mut(raw, len))
    }

    #[inline]
    pub fn into_raw_slice(this: &[Self]) -> &[MPI_Status] {
        unsafe { transmute(this) }
    }
    #[inline]
    pub fn into_raw_slice_mut(this: &mut [Self]) -> &mut [MPI_Status] {
        unsafe { transmute(this) }
    }
    #[inline]
    pub fn as_maybe_uninit_ptr(this: &[MaybeUninit<Self>]) -> *const MPI_Status {
        this.as_ptr() as *const _
    }
    #[inline]
    pub fn as_maybe_uninit_ptr_mut(this: &mut [MaybeUninit<Self>]) -> *mut MPI_Status {
        this.as_mut_ptr() as *mut _
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn get_count_with<F>(
            &self,
            get_count: F,
            datatype: &RawDatatype,
        ) -> RmpiResult<Option<c_int>>
        where
            F: FnOnce(*const MPI_Status, MPI_Datatype, *mut c_int) -> c_int,
        {
            let mut count = 0;
            Error::from_mpi_res(get_count(&self.into_raw(), datatype.as_raw(), &mut count)).map(
                |()| {
                    if count == MPI_UNDEFINED {
                        None
                    } else {
                        Some(count)
                    }
                },
            )
        }
    );
    #[inline]
    pub fn get_count(&self, datatype: &RawDatatype) -> RmpiResult<Option<c_int>> {
        unsafe {
            self.get_count_with(
                |status, datatype, count| MPI_Get_count(status, datatype, count),
                datatype,
            )
        }
    }
}

use std::{
    os::raw::{c_double, c_float, c_int, c_void},
    slice,
};

use mpi_sys::{
    MPI_Datatype, MPI_DOUBLE, MPI_FLOAT, MPI_INT16_T, MPI_INT32_T, MPI_INT64_T, MPI_INT8_T,
    MPI_UINT16_T, MPI_UINT32_T, MPI_UINT64_T, MPI_UINT8_T,
};

pub trait Buffer {
    fn item_datatype(&self) -> MPI_Datatype;
    fn into_raw(&self) -> (*const c_void, c_int);
    fn into_raw_mut(&mut self) -> (*mut c_void, c_int);
    unsafe fn from_raw<'b>(buf: *const c_void, count: c_int) -> &'b Self;
    unsafe fn from_raw_mut<'b>(buf: *mut c_void, count: c_int) -> &'b mut Self;
}
impl<T> Buffer for [T]
where
    T: MpiDatatype,
{
    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        T::datatype()
    }
    #[inline]
    fn into_raw(&self) -> (*const c_void, c_int) {
        let len = self.len();
        (self.as_ptr() as *mut c_void, len as c_int)
    }
    #[inline]
    fn into_raw_mut(&mut self) -> (*mut c_void, c_int) {
        let len = self.len();
        (self.as_mut_ptr() as *mut c_void, len as c_int)
    }
    #[inline]
    unsafe fn from_raw<'b>(buf: *const c_void, count: c_int) -> &'b Self {
        slice::from_raw_parts(buf as *const T, count as usize)
    }
    #[inline]
    unsafe fn from_raw_mut<'b>(buf: *mut c_void, count: c_int) -> &'b mut Self {
        slice::from_raw_parts_mut(buf as *mut T, count as usize)
    }
}

pub trait MpiDatatype {
    fn datatype() -> MPI_Datatype;
}

macro_rules! impl_mpi_datatype {
    ( $( $tp:ty : $dttp:expr ),* ) => {
        $(
            impl MpiDatatype for $tp {
                #[inline]
                fn datatype() -> MPI_Datatype {
                    $dttp
                }
            }
        )*
    };
}

impl_mpi_datatype!(
    u8: MPI_UINT8_T,
    u16: MPI_UINT16_T,
    u32: MPI_UINT32_T,
    u64: MPI_UINT64_T,
    i8: MPI_INT8_T,
    i16: MPI_INT16_T,
    i32: MPI_INT32_T,
    i64: MPI_INT64_T,
    c_float: MPI_FLOAT,
    c_double: MPI_DOUBLE
);

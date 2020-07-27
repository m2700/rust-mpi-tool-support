use std::{
    os::raw::{c_double, c_float, c_int, c_void},
    slice,
};

use cnum::Complex;

local_mod!(
    use mpi_sys::*;
);

pub trait Buffer {
    type PrimitiveElemType;
    type Single: SingleBuffer;

    fn item_datatype(&self) -> MPI_Datatype;

    fn into_raw(&self) -> (*const c_void, c_int);
    fn into_raw_mut(&mut self) -> (*mut c_void, c_int);

    fn as_ptr(&self) -> *const Self::PrimitiveElemType;
    fn as_mut_ptr(&mut self) -> *mut Self::PrimitiveElemType;

    unsafe fn from_raw<'b>(buf: *const c_void, count: c_int) -> &'b Self;
    unsafe fn from_raw_mut<'b>(buf: *mut c_void, count: c_int) -> &'b mut Self;

    fn as_bytes(&self) -> &[u8];
    fn len(&self) -> usize;
}
impl<T> Buffer for [T]
where
    T: MpiDatatype,
{
    type PrimitiveElemType = T;
    type Single = T;

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
    fn as_ptr(&self) -> *const Self::PrimitiveElemType {
        self.as_ptr()
    }
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::PrimitiveElemType {
        self.as_mut_ptr()
    }
    #[inline]
    unsafe fn from_raw<'b>(buf: *const c_void, count: c_int) -> &'b Self {
        slice::from_raw_parts(buf as *const T, count as usize)
    }
    #[inline]
    unsafe fn from_raw_mut<'b>(buf: *mut c_void, count: c_int) -> &'b mut Self {
        slice::from_raw_parts_mut(buf as *mut T, count as usize)
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe { self.align_to().1 }
    }
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

pub trait SingleBuffer: Buffer {
    unsafe fn from_raw_single<'b>(buf: *const c_void) -> Option<&'b Self>;
    unsafe fn from_raw_mut_single<'b>(buf: *mut c_void) -> Option<&'b mut Self>;
}
impl<T> Buffer for T
where
    T: MpiDatatype,
{
    type PrimitiveElemType = T;
    type Single = T;

    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        T::datatype()
    }
    #[inline]
    fn into_raw(&self) -> (*const c_void, c_int) {
        (self.as_ptr() as *mut c_void, 1)
    }
    #[inline]
    fn into_raw_mut(&mut self) -> (*mut c_void, c_int) {
        (self.as_mut_ptr() as *mut c_void, 1)
    }

    #[inline]
    fn as_ptr(&self) -> *const Self::PrimitiveElemType {
        self
    }
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::PrimitiveElemType {
        self
    }
    #[inline]
    unsafe fn from_raw<'b>(buf: *const c_void, count: c_int) -> &'b Self {
        assert_eq!(count, 1, "single buffer must have count 1");
        (buf as *const T)
            .as_ref()
            .expect("single buffer must not be nullptr")
    }
    #[inline]
    unsafe fn from_raw_mut<'b>(buf: *mut c_void, count: c_int) -> &'b mut Self {
        assert_eq!(count, 1, "single buffer must have count 1");
        (buf as *mut T)
            .as_mut()
            .expect("single buffer must not be nullptr")
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_ref(self).align_to().1 }
    }
    #[inline]
    fn len(&self) -> usize {
        1
    }
}
impl<T> SingleBuffer for T
where
    T: MpiDatatype,
{
    unsafe fn from_raw_single<'b>(buf: *const c_void) -> Option<&'b Self> {
        (buf as *const T).as_ref()
    }
    unsafe fn from_raw_mut_single<'b>(buf: *mut c_void) -> Option<&'b mut Self> {
        (buf as *mut T).as_mut()
    }
}

#[repr(transparent)]
pub struct CppBool(u8);
impl From<bool> for CppBool {
    #[inline]
    fn from(src: bool) -> Self {
        if src {
            Self(1)
        } else {
            Self(0)
        }
    }
}
impl From<CppBool> for bool {
    #[inline]
    fn from(src: CppBool) -> Self {
        match src {
            CppBool(0) => false,
            CppBool(_) => true,
        }
    }
}

/// is unsafe because it has to be transmutable to a byte array
pub unsafe trait MpiDatatype {
    fn datatype() -> MPI_Datatype;
}

macro_rules! impl_mpi_datatype {
    ( $( $tp:ty : $dttp:expr ),* ) => {
        $(
            unsafe impl MpiDatatype for $tp {
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
    c_double: MPI_DOUBLE,
    CppBool: MPI_C_BOOL,
    Complex<c_float>: MPI_C_FLOAT_COMPLEX,
    Complex<c_double>: MPI_C_DOUBLE_COMPLEX,
    LongInt: MPI_LONG_INT,
    DoubleInt: MPI_DOUBLE_INT,
    ShortInt: MPI_SHORT_INT,
    TwoInt: MPI_2INT,
    LongDoubleInt: MPI_LONG_DOUBLE_INT
);

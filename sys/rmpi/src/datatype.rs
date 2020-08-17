use std::{
    marker::PhantomData,
    mem::{forget, transmute},
    ops::Deref,
    os::raw::{c_double, c_float, c_int},
    ptr,
};

use cnum::Complex;

local_mod!(
    use mpi_sys::*;
    use crate::{Error, RmpiResult};
);

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

/// is unsafe because the type has to be transmutable to a byte array
pub unsafe trait MpiPredefinedDatatype {
    fn datatype() -> DatatypeRef<'static>;
}

macro_rules! unsafe_impl_mpi_predefined_datatype {
    ( $( $tp:ty : $dttp:expr ),* ) => {
        $(
            unsafe impl MpiPredefinedDatatype for $tp {
                #[inline]
                fn datatype() -> DatatypeRef<'static> {
                    unsafe{ DatatypeRef::from_raw_predefined($dttp) }
                }
            }
        )*
    };
}

unsafe_impl_mpi_predefined_datatype!(
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum DatatypeKind {
    Predefined,
    Custom,
}
use DatatypeKind::*;

/// Raw committed datatype.
/// Does not implement Copy or Clone,
/// as it would violate the lifetime restriction if the datatype is not mpi-predefined
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct RawDatatype(MPI_Datatype);
impl RawDatatype {
    #[inline]
    pub unsafe fn from_raw_ref(raw: &MPI_Datatype) -> &Self {
        transmute(raw)
    }
    /// only allowed if the datatype's lifetime is 'static
    #[inline]
    pub unsafe fn from_raw(raw: MPI_Datatype) -> Self {
        Self(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> MPI_Datatype {
        self.0
    }

    /// is safe because of the static lifetime
    #[inline]
    pub fn copy(&'static self) -> Self {
        Self(self.0)
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn size_with<F>(&self, mpi_type_size: F) -> RmpiResult<c_int>
        where
            F: FnOnce(MPI_Datatype, *mut c_int) -> c_int,
        {
            let mut size = 0;
            Error::from_mpi_res(mpi_type_size(self.0, &mut size))?;
            Ok(size)
        }
    );
    #[inline]
    pub fn size(&self) -> RmpiResult<c_int> {
        unsafe { self.size_with(|datatype, size| MPI_Type_size(datatype, size)) }
    }
}

/// datatype that is either predefined by mpi or already committed
#[derive(Eq, PartialEq)]
#[repr(C)]
pub struct Datatype {
    kind: DatatypeKind,
    raw: MPI_Datatype,
}
impl Drop for Datatype {
    #[inline]
    fn drop(&mut self) {
        unsafe { ptr::read(self).free().unwrap() }
    }
}
impl Default for Datatype {
    #[inline]
    fn default() -> Self {
        unsafe { Self::from_raw_predefined(MPI_DATATYPE_NULL) }
    }
}
impl Deref for Datatype {
    type Target = RawDatatype;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { RawDatatype::from_raw_ref(&self.raw) }
    }
}
impl Datatype {
    #[inline]
    pub unsafe fn from_raw_predefined(raw: MPI_Datatype) -> Self {
        Self {
            kind: Predefined,
            raw,
        }
    }
    /// assumes the handle to be comitted
    #[inline]
    pub unsafe fn from_raw_custom(raw: MPI_Datatype) -> Self {
        Self { kind: Custom, raw }
    }

    #[inline]
    pub fn as_ref(&self) -> DatatypeRef {
        DatatypeRef {
            kind: self.kind,
            raw: self.raw,
            lifetime: PhantomData,
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn free_with<F>(mut self, mpi_type_free: F) -> RmpiResult
        where
            F: FnOnce(*mut MPI_Datatype) -> c_int,
        {
            match self.kind {
                Predefined => Ok(()),
                Custom => {
                    let res = Error::from_mpi_res(mpi_type_free(&mut self.raw));
                    forget(self);
                    res
                }
            }
        }
    );
    #[inline]
    pub fn free(self) -> RmpiResult {
        unsafe { self.free_with(|tp| MPI_Type_free(tp)) }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn duplicate_with<F>(&self, mpi_type_dup: F) -> RmpiResult<Self>
        where
            F: FnOnce(MPI_Datatype, *mut MPI_Datatype) -> c_int,
        {
            match self.kind {
                Predefined => Ok(Self {
                    raw: self.raw,
                    kind: self.kind,
                }),
                Custom => {
                    let mut new_raw = MPI_DATATYPE_NULL;
                    let res = Error::from_mpi_res(mpi_type_dup(self.raw, &mut new_raw));
                    forget(self);
                    res.map(|()| Self {
                        raw: new_raw,
                        kind: self.kind,
                    })
                }
            }
        }
    );
    #[inline]
    pub fn duplicate(&self) -> RmpiResult<Self> {
        unsafe { self.duplicate_with(|oldtype, newtype| MPI_Type_dup(oldtype, newtype)) }
    }
}

/// fake reference onto a Datatype
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct DatatypeRef<'a> {
    kind: DatatypeKind,
    raw: MPI_Datatype,
    lifetime: PhantomData<&'a Datatype>,
}
impl Deref for DatatypeRef<'_> {
    type Target = Datatype;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute::<&DatatypeRef, &Datatype>(self) }
    }
}
impl Default for DatatypeRef<'_> {
    #[inline]
    fn default() -> Self {
        unsafe { Self::from_raw_predefined(MPI_DATATYPE_NULL) }
    }
}
impl DatatypeRef<'_> {
    #[inline]
    pub unsafe fn from_raw_predefined(raw: MPI_Datatype) -> Self {
        Self {
            kind: Predefined,
            raw,
            lifetime: PhantomData,
        }
    }
}

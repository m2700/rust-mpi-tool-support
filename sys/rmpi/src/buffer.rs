use std::{
    mem::size_of,
    os::raw::{c_double, c_float, c_int, c_void},
    slice,
};

use cnum::Complex;

local_mod!(
    use mpi_sys::*;
    use crate::{
        datatype::{MpiPredefinedDatatype, RawDatatype, CppBool},
        RmpiResult,
    };
);

pub trait BufferRef: Sized {
    type Mut: BufferMut;

    fn item_datatype(&self) -> MPI_Datatype;
    fn as_raw(&self) -> (*const c_void, c_int);
    fn as_ptr(&self) -> *const ();
    fn as_bytes(&self) -> &[u8];
    fn len(&self) -> usize;

    fn kind_ref(&self) -> BufferRefKind;

    #[inline]
    fn datatype_size(&self) -> RmpiResult<c_int> {
        unsafe { RawDatatype::from_raw(self.item_datatype()) }.size()
    }
}
pub trait BufferMut: Sized + BufferRef {
    type Ref: BufferRef;

    fn as_raw_mut(&mut self) -> (*mut c_void, c_int);
    fn as_mut_ptr(&mut self) -> *mut ();

    fn kind_mut(&mut self) -> BufferMutKind;
}

pub trait Buffer
where
    for<'b> &'b Self: BufferRef,
    for<'b> &'b mut Self: BufferMut,
{
    type PrimitiveElemType;

    fn as_typed_ptr(&self) -> *const Self::PrimitiveElemType;
    fn as_typed_mut_ptr(&mut self) -> *mut Self::PrimitiveElemType;

    unsafe fn from_raw<'b>(buf: *const c_void, count: c_int) -> &'b Self;
    unsafe fn from_raw_mut<'b>(buf: *mut c_void, count: c_int) -> &'b mut Self;
}
impl<T> Buffer for [T]
where
    T: MpiPredefinedDatatype,
{
    type PrimitiveElemType = T;

    #[inline]
    fn as_typed_ptr(&self) -> *const Self::PrimitiveElemType {
        self.as_ptr()
    }
    #[inline]
    fn as_typed_mut_ptr(&mut self) -> *mut Self::PrimitiveElemType {
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
}
impl<'b, T> BufferRef for &'b [T]
where
    T: MpiPredefinedDatatype,
{
    type Mut = &'b mut [T];

    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        T::datatype().as_raw()
    }
    #[inline]
    fn as_raw(&self) -> (*const c_void, c_int) {
        let len = self.len();
        (self.as_ptr() as *mut c_void, len as c_int)
    }
    #[inline]
    fn as_ptr(&self) -> *const () {
        (**self).as_ptr() as *const ()
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe { self.align_to().1 }
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn datatype_size(&self) -> RmpiResult<c_int> {
        Ok(size_of::<T>() as c_int)
    }
    #[inline]
    fn kind_ref(&self) -> BufferRefKind {
        T::buffer_ref_kind(self)
    }
}
impl<'b, T> BufferRef for &'b mut [T]
where
    T: MpiPredefinedDatatype,
{
    type Mut = &'b mut [T];

    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        T::datatype().as_raw()
    }
    #[inline]
    fn as_raw(&self) -> (*const c_void, c_int) {
        let len = self.len();
        (self.as_ptr() as *mut c_void, len as c_int)
    }
    #[inline]
    fn as_ptr(&self) -> *const () {
        (**self).as_ptr() as *const ()
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe { self.align_to().1 }
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn datatype_size(&self) -> RmpiResult<c_int> {
        Ok(size_of::<T>() as c_int)
    }
    #[inline]
    fn kind_ref(&self) -> BufferRefKind {
        T::buffer_ref_kind(self)
    }
}
impl<'b, T> BufferMut for &'b mut [T]
where
    T: MpiPredefinedDatatype,
{
    type Ref = &'b [T];

    #[inline]
    fn as_raw_mut(&mut self) -> (*mut c_void, c_int) {
        let len = self.len();
        (self.as_mut_ptr() as *mut c_void, len as c_int)
    }
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut () {
        (**self).as_mut_ptr() as *mut ()
    }
    #[inline]
    fn kind_mut(&mut self) -> BufferMutKind {
        T::buffer_mut_kind(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TypeDynamicBufferRef<'b> {
    /// has to be alligned correctly for datatype
    buffer: &'b [u8],
    /// has static lifetime, or lifetime is handled externally in unsafe block
    datatype: RawDatatype,
}
impl<'b> TypeDynamicBufferRef<'b> {
    #[inline]
    pub unsafe fn from_raw_dynamic(
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
    ) -> Self {
        let datatype = RawDatatype::from_raw(datatype);
        Self {
            buffer: <[u8]>::from_raw(
                buf,
                count * datatype.size().expect("could not get size of datatype"),
            ),
            datatype,
        }
    }
    #[inline]
    pub fn as_ref(&self) -> TypeDynamicBufferRef<'b> {
        *self
    }
}
impl<'b> BufferRef for TypeDynamicBufferRef<'b> {
    type Mut = TypeDynamicBufferMut<'b>;

    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        self.datatype.as_raw()
    }
    #[inline]
    fn as_raw(&self) -> (*const c_void, c_int) {
        self.buffer.as_raw()
    }
    #[inline]
    fn as_ptr(&self) -> *const () {
        self.buffer.as_ptr() as *const ()
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.buffer
    }
    #[inline]
    fn len(&self) -> usize {
        let size = self
            .datatype
            .size()
            .expect("could not get size of dynamic type") as usize;
        debug_assert_eq!(self.buffer.len() % size, 0);
        self.buffer.len() / size
    }
    #[inline]
    fn kind_ref(&self) -> BufferRefKind {
        BufferRefKind::TypeDynamic(*self)
    }
}
#[derive(Debug)]
pub struct TypeDynamicBufferMut<'b> {
    /// has to be alligned correctly for datatype
    buffer: &'b mut [u8],
    /// has static lifetime, or lifetime is handled externally in unsafe block
    datatype: RawDatatype,
}
impl<'b> TypeDynamicBufferMut<'b> {
    #[inline]
    pub unsafe fn from_raw_dynamic(buf: *mut c_void, count: c_int, datatype: MPI_Datatype) -> Self {
        let datatype = RawDatatype::from_raw(datatype);
        Self {
            buffer: <[u8]>::from_raw_mut(
                buf,
                count * datatype.size().expect("could not get size of datatype"),
            ),
            datatype,
        }
    }
    #[inline]
    pub fn as_mut(&mut self) -> TypeDynamicBufferMut {
        TypeDynamicBufferMut {
            buffer: self.buffer,
            datatype: self.datatype,
        }
    }
    #[inline]
    pub fn as_ref(&self) -> TypeDynamicBufferRef {
        TypeDynamicBufferRef {
            buffer: self.buffer,
            datatype: self.datatype,
        }
    }
}
impl<'b> BufferRef for TypeDynamicBufferMut<'b> {
    type Mut = TypeDynamicBufferMut<'b>;

    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        self.datatype.as_raw()
    }
    #[inline]
    fn as_raw(&self) -> (*const c_void, c_int) {
        self.buffer.as_raw()
    }
    #[inline]
    fn as_ptr(&self) -> *const () {
        self.buffer.as_ptr() as *const ()
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.buffer
    }
    #[inline]
    fn len(&self) -> usize {
        let size = self
            .datatype
            .size()
            .expect("could not get size of dynamic type") as usize;
        debug_assert_eq!(self.buffer.len() % size, 0);
        self.buffer.len() / size
    }
    #[inline]
    fn kind_ref(&self) -> BufferRefKind {
        BufferRefKind::TypeDynamic(self.as_ref())
    }
}
impl<'b> BufferMut for TypeDynamicBufferMut<'b> {
    type Ref = TypeDynamicBufferRef<'b>;

    #[inline]
    fn as_raw_mut(&mut self) -> (*mut c_void, c_int) {
        self.buffer.as_raw_mut()
    }
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut () {
        self.buffer.as_mut_ptr() as *mut ()
    }
    #[inline]
    fn kind_mut(&mut self) -> BufferMutKind {
        BufferMutKind::TypeDynamic(self.as_mut())
    }
}

impl<'a, B> BufferRef for &'a B
where
    B: BufferRef,
{
    type Mut = &'a mut B::Mut;

    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        (**self).item_datatype()
    }
    #[inline]
    fn as_raw(&self) -> (*const c_void, c_int) {
        (**self).as_raw()
    }
    #[inline]
    fn as_ptr(&self) -> *const () {
        (**self).as_ptr()
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        (**self).as_bytes()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn kind_ref(&self) -> BufferRefKind {
        (**self).kind_ref()
    }
}
impl<'a, B> BufferRef for &'a mut B
where
    B: BufferRef,
{
    type Mut = &'a mut B::Mut;

    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        (**self).item_datatype()
    }
    #[inline]
    fn as_raw(&self) -> (*const c_void, c_int) {
        (**self).as_raw()
    }
    #[inline]
    fn as_ptr(&self) -> *const () {
        (**self).as_ptr()
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        (**self).as_bytes()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
    #[inline]
    fn kind_ref(&self) -> BufferRefKind {
        (**self).kind_ref()
    }
}
impl<'a, B> BufferMut for &'a mut B
where
    B: BufferMut,
{
    type Ref = &'a B::Ref;

    #[inline]
    fn as_raw_mut(&mut self) -> (*mut c_void, c_int) {
        (**self).as_raw_mut()
    }
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut () {
        (**self).as_mut_ptr()
    }
    #[inline]
    fn kind_mut(&mut self) -> BufferMutKind {
        (**self).kind_mut()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BufferRefKind<'a> {
    U8(&'a [u8]),
    U16(&'a [u16]),
    U32(&'a [u32]),
    U64(&'a [u64]),
    I8(&'a [i8]),
    I16(&'a [i16]),
    I32(&'a [i32]),
    I64(&'a [i64]),
    Float(&'a [c_float]),
    Double(&'a [c_double]),
    CppBool(&'a [CppBool]),
    ComplexFloat(&'a [Complex<c_float>]),
    ComplexDouble(&'a [Complex<c_double>]),
    LongInt(&'a [LongInt]),
    DoubleInt(&'a [DoubleInt]),
    ShortInt(&'a [ShortInt]),
    TwoInt(&'a [TwoInt]),
    LongDoubleInt(&'a [LongDoubleInt]),
    TypeDynamic(TypeDynamicBufferRef<'a>),
}
#[derive(Debug)]
pub enum BufferMutKind<'a> {
    U8(&'a mut [u8]),
    U16(&'a mut [u16]),
    U32(&'a mut [u32]),
    U64(&'a mut [u64]),
    I8(&'a mut [i8]),
    I16(&'a mut [i16]),
    I32(&'a mut [i32]),
    I64(&'a mut [i64]),
    Float(&'a mut [c_float]),
    Double(&'a mut [c_double]),
    CppBool(&'a mut [CppBool]),
    ComplexFloat(&'a mut [Complex<c_float>]),
    ComplexDouble(&'a mut [Complex<c_double>]),
    LongInt(&'a mut [LongInt]),
    DoubleInt(&'a mut [DoubleInt]),
    ShortInt(&'a mut [ShortInt]),
    TwoInt(&'a mut [TwoInt]),
    LongDoubleInt(&'a mut [LongDoubleInt]),
    TypeDynamic(TypeDynamicBufferMut<'a>),
}

use std::{
    mem::size_of,
    os::raw::{c_int, c_void},
    slice,
};

local_mod!(
    use mpi_sys::*;
    use crate::{
        datatype::{MpiPredefinedDatatype, RawDatatype},
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

    #[inline]
    fn datatype_size(&self) -> RmpiResult<c_int> {
        unsafe { RawDatatype::from_raw(self.item_datatype()) }.size()
    }
}
pub trait BufferMut: Sized + BufferRef {
    type Ref: BufferRef;

    fn as_raw_mut(&mut self) -> (*mut c_void, c_int);
    fn as_mut_ptr(&mut self) -> *mut ();
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
}

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
}
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
}

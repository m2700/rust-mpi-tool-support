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
    // type Single: SingleBufferRef;

    fn item_datatype(&self) -> MPI_Datatype;
    fn as_raw(&self) -> (*const c_void, c_int);
    fn as_ptr(&self) -> *const ();
    unsafe fn from_raw2(buf: *const c_void, count: c_int, datatype: MPI_Datatype) -> Self;
    fn as_bytes(&self) -> &[u8];
    fn len(&self) -> usize;

    #[inline]
    fn datatype_size(&self) -> RmpiResult<c_int> {
        unsafe { RawDatatype::from_raw(self.item_datatype()) }.size()
    }
}
pub trait BufferMut: Sized {
    type Ref: BufferRef;
    // type Single: SingleBufferMut;

    fn item_datatype(&self) -> MPI_Datatype;
    fn as_raw_mut(&mut self) -> (*mut c_void, c_int);
    fn as_mut_ptr(&mut self) -> *mut ();
    unsafe fn from_raw_mut2(buf: *mut c_void, count: c_int, datatype: MPI_Datatype) -> Self;
    fn as_bytes(&self) -> &[u8];
    fn len(&self) -> usize;

    #[inline]
    fn datatype_size(&self) -> RmpiResult<c_int> {
        unsafe { RawDatatype::from_raw(self.item_datatype()) }.size()
    }
}

pub trait Buffer {
    type PrimitiveElemType;
    // type Single: SingleBuffer;

    fn item_datatype(&self) -> MPI_Datatype;

    fn into_raw(&self) -> (*const c_void, c_int);
    fn into_raw_mut(&mut self) -> (*mut c_void, c_int);

    fn as_ptr(&self) -> *const Self::PrimitiveElemType;
    fn as_mut_ptr(&mut self) -> *mut Self::PrimitiveElemType;

    unsafe fn from_raw<'b>(buf: *const c_void, count: c_int) -> &'b Self;
    unsafe fn from_raw_mut<'b>(buf: *mut c_void, count: c_int) -> &'b mut Self;

    fn as_bytes(&self) -> &[u8];
    fn len(&self) -> usize;

    fn datatype_size(&self) -> RmpiResult<c_int>;
}
impl<T> Buffer for [T]
where
    T: MpiPredefinedDatatype,
{
    type PrimitiveElemType = T;
    // type Single = T;

    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        T::datatype().as_raw()
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
    #[inline]
    fn datatype_size(&self) -> RmpiResult<c_int> {
        Ok(size_of::<T>() as c_int)
    }
}
impl<'b, B: Buffer + ?Sized> BufferRef for &'b B {
    type Mut = &'b mut B;
    // type Single = &'b B::Single;
    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        <B as Buffer>::item_datatype(self)
    }
    #[inline]
    fn as_raw(&self) -> (*const c_void, c_int) {
        <B as Buffer>::into_raw(self)
    }
    #[inline]
    fn as_ptr(&self) -> *const () {
        <B as Buffer>::as_ptr(self) as *const ()
    }
    #[inline]
    unsafe fn from_raw2(buf: *const c_void, count: c_int, _datatype: MPI_Datatype) -> Self {
        <B as Buffer>::from_raw(buf, count)
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        <B as Buffer>::as_bytes(self)
    }
    #[inline]
    fn len(&self) -> usize {
        <B as Buffer>::len(self)
    }
    #[inline]
    fn datatype_size(&self) -> RmpiResult<c_int> {
        <B as Buffer>::datatype_size(self)
    }
}
impl<'b, B: Buffer + ?Sized> BufferMut for &'b mut B {
    type Ref = &'b B;
    // type Single = &'b mut B::Single;
    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        <B as Buffer>::item_datatype(self)
    }
    #[inline]
    fn as_raw_mut(&mut self) -> (*mut c_void, c_int) {
        <B as Buffer>::into_raw_mut(self)
    }
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut () {
        <B as Buffer>::as_mut_ptr(self) as *mut ()
    }
    #[inline]
    unsafe fn from_raw_mut2(buf: *mut c_void, count: c_int, _datatype: MPI_Datatype) -> Self {
        <B as Buffer>::from_raw_mut(buf, count)
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        <B as Buffer>::as_bytes(self)
    }
    #[inline]
    fn len(&self) -> usize {
        <B as Buffer>::len(self)
    }
    #[inline]
    fn datatype_size(&self) -> RmpiResult<c_int> {
        <B as Buffer>::datatype_size(self)
    }
}

pub struct TypeDynamicBufferRef<'b> {
    /// has to be alligned correctly for datatype
    buffer: &'b [u8],
    /// has static lifetime, or lifetime is handled externally in unsafe block
    datatype: RawDatatype,
}
impl<'b> BufferRef for TypeDynamicBufferRef<'b> {
    type Mut = TypeDynamicBufferMut<'b>;
    // type Single = TypeDynamicSingleBufferRef<'b>;
    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        self.datatype.as_raw()
    }
    #[inline]
    fn as_raw(&self) -> (*const c_void, c_int) {
        self.buffer.into_raw()
    }
    #[inline]
    fn as_ptr(&self) -> *const () {
        self.buffer.as_ptr() as *const ()
    }
    #[inline]
    unsafe fn from_raw2(buf: *const c_void, count: c_int, datatype: MPI_Datatype) -> Self {
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
impl<'b> BufferMut for TypeDynamicBufferMut<'b> {
    type Ref = TypeDynamicBufferRef<'b>;
    // type Single = TypeDynamicSingleBufferMut<'b>;
    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        self.datatype.as_raw()
    }
    #[inline]
    fn as_raw_mut(&mut self) -> (*mut c_void, c_int) {
        self.buffer.into_raw_mut()
    }
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut () {
        self.buffer.as_mut_ptr() as *mut ()
    }
    #[inline]
    unsafe fn from_raw_mut2(buf: *mut c_void, count: c_int, datatype: MPI_Datatype) -> Self {
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

// pub trait SingleBufferRef: BufferRef + Sized {
//     #[inline]
//     unsafe fn from_raw_single(buf: *const c_void, datatype: MPI_Datatype) -> Option<Self> {
//         if buf.is_null() {
//             None
//         } else {
//             Some(<Self as BufferRef>::from_raw2(buf, 1, datatype))
//         }
//     }
// }
// pub trait SingleBufferMut: BufferMut + Sized {
//     #[inline]
//     unsafe fn from_raw_mut_single(buf: *mut c_void, datatype: MPI_Datatype) -> Option<Self> {
//         if buf.is_null() {
//             None
//         } else {
//             Some(<Self as BufferMut>::from_raw_mut2(buf, 1, datatype))
//         }
//     }
// }
// pub trait SingleBuffer: Buffer {
//     unsafe fn from_raw_single<'b>(buf: *const c_void) -> Option<&'b Self>;
//     unsafe fn from_raw_mut_single<'b>(buf: *mut c_void) -> Option<&'b mut Self>;
// }
// impl<T> Buffer for T
// where
//     T: MpiPredefinedDatatype,
// {
//     type PrimitiveElemType = T;
//     type Single = T;

//     #[inline]
//     fn item_datatype(&self) -> MPI_Datatype {
//         T::datatype().as_raw()
//     }
//     #[inline]
//     fn into_raw(&self) -> (*const c_void, c_int) {
//         (self.as_ptr() as *mut c_void, 1)
//     }
//     #[inline]
//     fn into_raw_mut(&mut self) -> (*mut c_void, c_int) {
//         (self.as_mut_ptr() as *mut c_void, 1)
//     }

//     #[inline]
//     fn as_ptr(&self) -> *const Self::PrimitiveElemType {
//         self
//     }
//     #[inline]
//     fn as_mut_ptr(&mut self) -> *mut Self::PrimitiveElemType {
//         self
//     }
//     #[inline]
//     unsafe fn from_raw<'b>(buf: *const c_void, count: c_int) -> &'b Self {
//         assert_eq!(count, 1, "single buffer must have count 1");
//         (buf as *const T)
//             .as_ref()
//             .expect("single buffer must not be nullptr")
//     }
//     #[inline]
//     unsafe fn from_raw_mut<'b>(buf: *mut c_void, count: c_int) -> &'b mut Self {
//         assert_eq!(count, 1, "single buffer must have count 1");
//         (buf as *mut T)
//             .as_mut()
//             .expect("single buffer must not be nullptr")
//     }

//     #[inline]
//     fn as_bytes(&self) -> &[u8] {
//         unsafe { slice::from_ref(self).align_to().1 }
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         1
//     }
//     #[inline]
//     fn datatype_size(&self) -> RmpiResult<c_int> {
//         Ok(size_of::<T>() as c_int)
//     }
// }
// impl<T> SingleBuffer for T
// where
//     T: MpiPredefinedDatatype,
// {
//     unsafe fn from_raw_single<'b>(buf: *const c_void) -> Option<&'b Self> {
//         (buf as *const T).as_ref()
//     }
//     unsafe fn from_raw_mut_single<'b>(buf: *mut c_void) -> Option<&'b mut Self> {
//         (buf as *mut T).as_mut()
//     }
// }
// impl<'b, B: SingleBuffer> SingleBufferRef for &'b B {}
// impl<'b, B: SingleBuffer> SingleBufferMut for &'b mut B {}

// pub struct TypeDynamicSingleBufferRef<'b> {
//     /// has to be alligned correctly for datatype, and has to have the correct size
//     buffer: &'b [u8],
//     /// has static lifetime, or lifetime is handled externally in unsafe block
//     datatype: RawDatatype,
// }
// impl<'b> SingleBufferRef for TypeDynamicSingleBufferRef<'b> {}
// impl<'b> BufferRef for TypeDynamicSingleBufferRef<'b> {
//     type Mut = TypeDynamicSingleBufferMut<'b>;
//     type Single = TypeDynamicSingleBufferRef<'b>;
//     #[inline]
//     fn item_datatype(&self) -> MPI_Datatype {
//         self.datatype.as_raw()
//     }
//     #[inline]
//     fn as_raw(&self) -> (*const c_void, c_int) {
//         self.buffer.into_raw()
//     }
//     #[inline]
//     fn as_ptr(&self) -> *const () {
//         self.buffer.as_ptr() as *const ()
//     }
//     #[inline]
//     unsafe fn from_raw2(buf: *const c_void, count: c_int, datatype: MPI_Datatype) -> Self {
//         let datatype = RawDatatype::from_raw(datatype);
//         let datatype_size = datatype.size().expect("could not get size of dynamic type");
//         let buffer = <[u8]>::from_raw(buf, count * datatype_size);
//         debug_assert_eq!(count, 1);
//         debug_assert_eq!(
//             buffer.len() as c_int,
//             datatype_size,
//             "single-buffer must have size of datatype"
//         );
//         Self { buffer, datatype }
//     }
//     #[inline]
//     fn as_bytes(&self) -> &[u8] {
//         self.buffer
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         let size = self
//             .datatype
//             .size()
//             .expect("could not get size of dynamic type") as usize;
//         debug_assert_eq!(self.buffer.len() % size, 0);
//         self.buffer.len() / size
//     }
// }
// pub struct TypeDynamicSingleBufferMut<'b> {
//     /// has to be alligned correctly for datatype, and has to have the correct size
//     buffer: &'b mut [u8],
//     /// has static lifetime, or lifetime is handled externally in unsafe block
//     datatype: RawDatatype,
// }
// impl<'b> SingleBufferMut for TypeDynamicSingleBufferMut<'b> {}
// impl<'b> BufferMut for TypeDynamicSingleBufferMut<'b> {
//     type Ref = TypeDynamicSingleBufferRef<'b>;
//     type Single = TypeDynamicSingleBufferMut<'b>;
//     #[inline]
//     fn item_datatype(&self) -> MPI_Datatype {
//         self.datatype.as_raw()
//     }
//     #[inline]
//     fn as_raw_mut(&mut self) -> (*mut c_void, c_int) {
//         self.buffer.into_raw_mut()
//     }
//     #[inline]
//     fn as_mut_ptr(&mut self) -> *mut () {
//         self.buffer.as_mut_ptr() as *mut ()
//     }
//     #[inline]
//     unsafe fn from_raw_mut2(buf: *mut c_void, count: c_int, datatype: MPI_Datatype) -> Self {
//         let datatype = RawDatatype::from_raw(datatype);
//         let datatype_size = datatype.size().expect("could not get size of dynamic type");
//         let buffer = <[u8]>::from_raw_mut(buf, count * datatype_size);
//         debug_assert_eq!(count, 1);
//         debug_assert_eq!(
//             buffer.len() as c_int,
//             datatype_size,
//             "single-buffer must have size of datatype"
//         );
//         Self { buffer, datatype }
//     }
//     #[inline]
//     fn as_bytes(&self) -> &[u8] {
//         self.buffer
//     }
//     #[inline]
//     fn len(&self) -> usize {
//         let size = self
//             .datatype
//             .size()
//             .expect("could not get size of dynamic type") as usize;
//         debug_assert_eq!(self.buffer.len() % size, 0);
//         self.buffer.len() / size
//     }
// }

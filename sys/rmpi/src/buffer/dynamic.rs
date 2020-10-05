use std::os::raw::{c_double, c_float, c_int, c_void};

local_mod!(
    use mpi_sys::*;
    use crate::{
        datatype::{MpiPredefinedDatatype, RawDatatype},
    };
);

use super::*;

/// A Buffer-Reference implementation where the Datatype is only known as runtime.
#[derive(Debug, Clone, Copy)]
pub struct TypeDynamicBufferRef<'b> {
    /// has to be alligned correctly for datatype
    buffer: &'b [u8],
    /// has 'static lifetime, or lifetime is handled externally in unsafe block
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
        let len = self.len();
        (self.as_ptr() as *const c_void, len as c_int)
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
        define_datatype!(
            type Datatype = self.datatype.as_raw();
            {
                let (ptr, len) = self.as_raw();
                let typed_buffer = unsafe {
                    <[Datatype] as Buffer>::from_raw(ptr, len)
                };
                Datatype::buffer_ref_kind(typed_buffer)
            } dynamic {
                BufferRefKind::TypeDynamic(*self)
            }
        )
    }
}

/// A Mutable Buffer-Reference implementation where the Datatype is only known as runtime.
#[derive(Debug)]
pub struct TypeDynamicBufferMut<'b> {
    /// has to be alligned correctly for datatype
    buffer: &'b mut [u8],
    /// has 'static lifetime, or lifetime is handled externally in unsafe block
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

    #[inline]
    pub(crate) fn slice_to_mut(self, size: usize) -> TypeDynamicBufferMut<'b> {
        let datatype_size = self
            .datatype
            .size()
            .expect("could not get size of dynamic type") as usize;
        TypeDynamicBufferMut {
            buffer: &mut self.buffer[..(size * datatype_size)],
            datatype: self.datatype,
        }
    }
}
impl<'b> BufferRef for TypeDynamicBufferMut<'b> {
    type Mut = TypeDynamicBufferMut<'b>;

    #[inline]
    fn item_datatype(&self) -> MPI_Datatype {
        self.as_ref().item_datatype()
    }
    #[inline]
    fn as_raw(&self) -> (*const c_void, c_int) {
        self.as_ref().as_raw()
    }
    #[inline]
    fn as_ptr(&self) -> *const () {
        self.as_ref().as_ptr()
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.buffer
    }
    #[inline]
    fn len(&self) -> usize {
        self.as_ref().len()
    }
    #[inline]
    fn kind_ref(&self) -> BufferRefKind {
        define_datatype!(
            type Datatype = self.datatype.as_raw();
            {
                let (ptr, len) = self.as_raw();
                let typed_buffer = unsafe {
                    <[Datatype] as Buffer>::from_raw(ptr, len)
                };
                Datatype::buffer_ref_kind(typed_buffer)
            } dynamic {
                BufferRefKind::TypeDynamic(self.as_ref())
            }
        )
    }
}
impl<'b> BufferMut for TypeDynamicBufferMut<'b> {
    type Ref = TypeDynamicBufferRef<'b>;

    #[inline]
    fn as_raw_mut(&mut self) -> (*mut c_void, c_int) {
        let len = self.len();
        (self.as_mut_ptr() as *mut c_void, len as c_int)
    }
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut () {
        self.buffer.as_mut_ptr() as *mut ()
    }
    #[inline]
    fn kind_mut(&mut self) -> BufferMutKind {
        define_datatype!(
            type Datatype = self.datatype.as_raw();
            {
                let (ptr, len) = self.as_raw_mut();
                let typed_buffer = unsafe {
                    <[Datatype] as Buffer>::from_raw_mut(ptr, len)
                };
                Datatype::buffer_mut_kind(typed_buffer)
            } dynamic {
                BufferMutKind::TypeDynamic(self.as_mut())
            }
        )
    }
}

use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::{forget, transmute},
    os::raw::{c_char, c_int},
    ptr::{self},
    slice,
};

local_mod!(
    use mpi_sys::*;
    use crate::{Error, RmpiResult};
);

#[repr(transparent)]
pub struct CStrMutPtr<'a> {
    ptr: *mut c_char,
    lifetime: PhantomData<&'a mut CStr>,
}
impl<'a> CStrMutPtr<'a> {
    #[inline]
    pub fn null() -> Self {
        Self {
            ptr: ptr::null_mut(),
            lifetime: PhantomData,
        }
    }
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_char) -> Self {
        Self {
            ptr,
            lifetime: PhantomData,
        }
    }
    #[inline]
    pub fn borrow_from_c_string(c_string: &'a mut CString) -> Self {
        unsafe { Self::from_raw(ptr::read(c_string).into_raw()) }
    }
    #[inline]
    pub fn as_cstr(&self) -> Option<&CStr> {
        unsafe { self.ptr.as_ref().map(|rf| CStr::from_ptr(rf)) }
    }
}

tool_mode_item!(
    #[inline]
    pub unsafe fn initialized_with<F>(mpi_initialized: F) -> RmpiResult<bool>
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        let mut flag = 0;
        Error::from_mpi_res(mpi_initialized(&mut flag)).map(|()| flag != 0)
    }
);
#[inline]
pub fn initialized() -> RmpiResult<bool> {
    unsafe { initialized_with(|flag| MPI_Initialized(flag)) }
}

tool_mode_item!(
    #[inline]
    pub unsafe fn finalized_with<F>(mpi_finalized: F) -> RmpiResult<bool>
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        initialized_with(mpi_finalized)
    }
);
#[inline]
pub fn finalized() -> RmpiResult<bool> {
    unsafe { finalized_with(|flag| MPI_Finalized(flag)) }
}

tool_mode_item!(
    #[inline]
    pub unsafe fn init_with<F>(
        mpi_init: F,
        args_mut: &mut &mut [CStrMutPtr],
    ) -> RmpiResult<Option<RmpiContext>>
    where
        F: FnOnce(*mut c_int, *mut *mut *mut c_char) -> c_int,
    {
        if initialized()? {
            return Ok(None);
        }
        let args = &mut **args_mut;
        let args = transmute::<&mut [CStrMutPtr], &mut [*mut c_char]>(args);
        let mut argv = args.as_mut_ptr();
        let mut argc = args.len() as c_int;
        Error::from_mpi_res(mpi_init(&mut argc, &mut argv)).map(|()| {
            *args_mut = match argv.as_mut() {
                Some(argv) => transmute::<&mut [*mut c_char], &mut [CStrMutPtr]>(
                    slice::from_raw_parts_mut(argv, argc as usize),
                ),
                None => &mut [],
            };
            Some(RmpiContext {
                not_send_marker: PhantomData,
            })
        })
    }
);
#[inline]
pub fn init(args_mut: &mut &mut [CStrMutPtr]) -> RmpiResult<Option<RmpiContext>> {
    unsafe { init_with(|argc, argv| MPI_Init(argc, argv), args_mut) }
}

pub struct RmpiContext {
    not_send_marker: PhantomData<*mut ()>,
}
impl Drop for RmpiContext {
    #[inline]
    fn drop(&mut self) {
        Self {
            not_send_marker: self.not_send_marker,
        }
        .finalize()
        .unwrap()
    }
}
impl RmpiContext {
    #[inline]
    pub fn finalize(self) -> RmpiResult {
        forget(self);
        Error::from_mpi_res(unsafe { MPI_Finalize() })
    }
}
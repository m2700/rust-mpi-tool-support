use std::{mem::MaybeUninit, os::raw::c_int};

use mpi_sys::*;

use crate::{Error, RmpiResult, Status};

#[repr(transparent)]
pub struct Request(MPI_Request);
impl Request {
    #[inline]
    pub const unsafe fn from_raw(raw: MPI_Request) -> Self {
        Self(raw)
    }
    #[inline]
    pub const fn into_raw(self) -> MPI_Request {
        self.0
    }

    tool_mode_item! {
        #[inline]
        pub unsafe fn wait_with<F>(&mut self, mpi_wait: F) -> RmpiResult<Status>
            where F: FnOnce(*mut MPI_Request, *mut MPI_Status) -> c_int
        {
            let mut status = MaybeUninit::uninit();
            Error::from_mpi_res(mpi_wait(&mut self.0, status.as_mut_ptr()))
                .map(|()| Status::from_raw(status.assume_init()))
        }
    }
    #[inline]
    pub fn wait(mut self) -> RmpiResult<Status> {
        unsafe { self.wait_with(|request, status| MPI_Wait(request, status)) }
    }

    tool_mode_item! {
        #[inline]
        pub unsafe fn test_with<F>(&mut self, mpi_test: F) -> RmpiResult<Option<Status>>
            where F: FnOnce(*mut MPI_Request, *mut c_int, *mut MPI_Status) -> c_int
        {
            let mut status = MaybeUninit::uninit();
            let mut flag = 0;
            Error::from_mpi_res(mpi_test(&mut self.0, &mut flag, status.as_mut_ptr()))
                .map(|()| {
                    if flag != 0 {
                        Some(Status::from_raw(status.assume_init()))
                    } else { None }
                })
        }
    }
    #[inline]
    pub fn test(&mut self) -> RmpiResult<Option<Status>> {
        unsafe { self.test_with(|request, flag, status| MPI_Test(request, flag, status)) }
    }
}

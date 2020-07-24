use std::{
    mem::{forget, transmute, MaybeUninit},
    ops::{Deref, DerefMut},
    os::raw::c_int,
};

local_mod!(
    use mpi_sys::*;
    use crate::{Error, RmpiResult, Status};
);

#[repr(transparent)]
pub struct Request(MPI_Request);
impl Drop for Request {
    #[inline]
    fn drop(&mut self) {
        Self(self.0).free().unwrap()
    }
}
impl Request {
    #[inline]
    pub const unsafe fn from_raw(raw: MPI_Request) -> Self {
        Self(raw)
    }
    #[inline]
    pub fn into_raw(self) -> MPI_Request {
        let raw = self.0;
        forget(self);
        raw
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn wait_with<F>(&mut self, mpi_wait: F) -> RmpiResult<Status>
        where
            F: FnOnce(*mut MPI_Request, *mut MPI_Status) -> c_int,
        {
            let mut status = MaybeUninit::uninit();
            Error::from_mpi_res(mpi_wait(&mut self.0, status.as_mut_ptr()))
                .map(|()| Status::from_raw(status.assume_init()))
        }
    );
    #[inline]
    pub fn wait(&mut self) -> RmpiResult<Status> {
        unsafe { self.wait_with(|request, status| MPI_Wait(request, status)) }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn test_with<F>(&mut self, mpi_test: F) -> RmpiResult<Option<Status>>
        where
            F: FnOnce(*mut MPI_Request, *mut c_int, *mut MPI_Status) -> c_int,
        {
            let mut status = MaybeUninit::uninit();
            let mut flag = 0;
            Error::from_mpi_res(mpi_test(&mut self.0, &mut flag, status.as_mut_ptr())).map(|()| {
                if flag != 0 {
                    Some(Status::from_raw(status.assume_init()))
                } else {
                    None
                }
            })
        }
    );
    #[inline]
    pub fn test(&mut self) -> RmpiResult<Option<Status>> {
        unsafe { self.test_with(|request, flag, status| MPI_Test(request, flag, status)) }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn free_with<F>(mut self, mpi_free: F) -> RmpiResult
        where
            F: FnOnce(*mut MPI_Request) -> c_int,
        {
            let res = Error::from_mpi_res(mpi_free(&mut self.0));
            forget(self);
            res
        }
    );
    #[inline]
    pub fn free(self) -> RmpiResult {
        unsafe { self.free_with(|request| MPI_Request_free(request)) }
    }
}

#[repr(transparent)]
pub struct RequestSlice([MPI_Request]);
impl<'a> From<&'a [Request]> for &'a RequestSlice {
    #[inline]
    fn from(src: &'a [Request]) -> Self {
        unsafe { transmute(src) }
    }
}
impl<'a> From<&'a mut [Request]> for &'a mut RequestSlice {
    #[inline]
    fn from(src: &'a mut [Request]) -> Self {
        unsafe { transmute(src) }
    }
}
impl RequestSlice {
    #[inline]
    pub unsafe fn from_raw(slc: &[MPI_Request]) -> &Self {
        transmute(slc)
    }
    #[inline]
    pub unsafe fn from_raw_mut(slc: &mut [MPI_Request]) -> &mut Self {
        transmute(slc)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn waitany_with<F>(&mut self, mpi_waitany: F) -> RmpiResult<(usize, Status)>
        where
            F: FnOnce(c_int, *mut MPI_Request, *mut c_int, *mut MPI_Status) -> c_int,
        {
            let mut indx = 0;
            let mut status = MaybeUninit::uninit();
            Error::from_mpi_res(mpi_waitany(
                self.0.len() as c_int,
                self.0.as_mut_ptr(),
                &mut indx,
                status.as_mut_ptr(),
            ))
            .map(|()| (indx as usize, Status::from_raw(status.assume_init())))
        }
    );
    #[inline]
    pub fn waitany(&mut self) -> RmpiResult<(usize, Status)> {
        unsafe {
            self.waitany_with(|count, array_of_requests, indx, status| {
                MPI_Waitany(count, array_of_requests, indx, status)
            })
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn testany_with<F>(
            &mut self,
            mpi_testany: F,
        ) -> RmpiResult<Option<(usize, Status)>>
        where
            F: FnOnce(c_int, *mut MPI_Request, *mut c_int, *mut c_int, *mut MPI_Status) -> c_int,
        {
            let mut indx = 0;
            let mut flag = 0;
            let mut status = MaybeUninit::uninit();
            Error::from_mpi_res(mpi_testany(
                self.0.len() as c_int,
                self.0.as_mut_ptr(),
                &mut indx,
                &mut flag,
                status.as_mut_ptr(),
            ))
            .map(|()| {
                if flag != 0 {
                    Some((indx as usize, Status::from_raw(status.assume_init())))
                } else {
                    None
                }
            })
        }
    );
    #[inline]
    pub fn testany(&mut self) -> RmpiResult<Option<(usize, Status)>> {
        unsafe {
            self.testany_with(|count, array_of_requests, indx, flag, status| {
                MPI_Testany(count, array_of_requests, indx, flag, status)
            })
        }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn waitall_with<F>(
            &mut self,
            mpi_waitall: F,
            statuses: &mut [Status],
        ) -> RmpiResult
        where
            F: FnOnce(c_int, *mut MPI_Request, *mut MPI_Status) -> c_int,
        {
            debug_assert_eq!(self.len(), statuses.len());
            Error::from_mpi_res(mpi_waitall(
                self.0.len() as c_int,
                self.0.as_mut_ptr(),
                Status::into_raw_slice_mut(statuses).as_mut_ptr(),
            ))
        }
    );
    #[inline]
    pub fn waitall(&mut self, statuses: &mut [Status]) -> RmpiResult {
        unsafe {
            self.waitall_with(
                |count, array_of_requests, array_of_statuses| {
                    MPI_Waitall(count, array_of_requests, array_of_statuses)
                },
                statuses,
            )
        }
    }
}

#[repr(transparent)]
pub struct RequestVec(Vec<MPI_Request>);
impl<'a> From<Vec<Request>> for RequestVec {
    #[inline]
    fn from(src: Vec<Request>) -> Self {
        unsafe { transmute(src) }
    }
}
impl Deref for RequestVec {
    type Target = RequestSlice;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute::<&[MPI_Request], &RequestSlice>(&*self.0) }
    }
}
impl DerefMut for RequestVec {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute::<&mut [MPI_Request], &mut RequestSlice>(&mut *self.0) }
    }
}
impl RequestVec {
    #[inline]
    pub fn push(&mut self, request: Request) {
        self.0.push(request.into_raw())
    }
}

use std::{
    marker::PhantomData,
    mem::{forget, transmute, MaybeUninit},
    ops::{Deref, DerefMut},
    os::raw::c_int,
};

local_mod!(
    use mpi_sys::*;
    use crate::{Error, RmpiResult, Status};
);

/// Possibly Persistant Request, returned by a isend, irecv or similar operation.
/// The lifetime 'b refers to the buffer that is used in the argument
/// of the respective operation and is therefore either a mutable or immutable borrow
/// (despite immutable &'b [u8])
/// The Request needs to be freed, canceled, or awaited manually,
/// before the request goes out of scope. A panic!() occures otherwise.
#[repr(transparent)]
pub struct Request<'b>(MPI_Request, PhantomData<&'b [u8]>);
impl<'b> Drop for Request<'b> {
    #[inline]
    fn drop(&mut self) {
        panic!("request needs to be freed, canceled, or awaited")
    }
}
impl<'b> Request<'b> {
    #[inline]
    pub const unsafe fn from_raw(raw: MPI_Request) -> Self {
        Self(raw, PhantomData)
    }
    #[inline]
    pub fn into_raw(self) -> MPI_Request {
        let raw = self.0;
        forget(self);
        raw
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn wait_with<F>(mut self, mpi_wait: F) -> RmpiResult<(Status, Option<Self>)>
        where
            F: FnOnce(*mut MPI_Request, *mut MPI_Status) -> c_int,
        {
            let mut status = MaybeUninit::uninit();
            match Error::from_mpi_res(mpi_wait(&mut self.0, status.as_mut_ptr())) {
                Ok(()) => Ok((
                    Status::from_raw(status.assume_init()),
                    if self.0 == MPI_REQUEST_NULL {
                        forget(self);
                        None
                    } else {
                        Some(self)
                    },
                )),
                Err(e) => {
                    forget(self);
                    Err(e)
                }
            }
        }
    );
    #[inline]
    pub fn wait(self) -> RmpiResult<(Status, Option<Self>)> {
        unsafe { self.wait_with(|request, status| MPI_Wait(request, status)) }
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn test_with<F>(
            mut self,
            mpi_test: F,
        ) -> RmpiResult<Result<(Status, Option<Self>), Self>>
        where
            F: FnOnce(*mut MPI_Request, *mut c_int, *mut MPI_Status) -> c_int,
        {
            let mut status = MaybeUninit::uninit();
            let mut flag = 0;
            match Error::from_mpi_res(mpi_test(&mut self.0, &mut flag, status.as_mut_ptr())) {
                Ok(()) => Ok(if flag != 0 {
                    Ok((
                        Status::from_raw(status.assume_init()),
                        if self.0 == MPI_REQUEST_NULL {
                            forget(self);
                            None
                        } else {
                            Some(self)
                        },
                    ))
                } else {
                    Err(self)
                }),
                Err(e) => {
                    forget(self);
                    Err(e)
                }
            }
        }
    );
    #[inline]
    pub fn test(self) -> RmpiResult<Result<(Status, Option<Self>), Self>> {
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
    /// Frees the request, regardless if finished or not.
    /// This is unsafe, because it does not stop the underlying
    /// operation from happening and the buffer might therefore still be in use.
    /// Use cancel() or wait(), instead, to prevent that.
    #[inline]
    pub unsafe fn free(self) -> RmpiResult {
        self.free_with(|request| MPI_Request_free(request))
    }

    tool_mode_item!(
        #[inline]
        pub unsafe fn cancel_with<F>(self, mpi_cancel: F) -> RmpiResult
        where
            F: FnOnce(*mut MPI_Request) -> c_int,
        {
            self.free_with(mpi_cancel)
        }
    );
    #[inline]
    pub fn cancel(self) -> RmpiResult {
        unsafe { self.cancel_with(|request| MPI_Cancel(request)) }
    }
}

#[repr(transparent)]
pub struct RequestSlice<'b>(PhantomData<&'b [u8]>, [MPI_Request]);
impl<'a, 'b> From<&'a [Request<'b>]> for &'a RequestSlice<'b> {
    #[inline]
    fn from(src: &'a [Request<'b>]) -> Self {
        unsafe { transmute(src) }
    }
}
impl<'a, 'b> From<&'a mut [Request<'b>]> for &'a mut RequestSlice<'b> {
    #[inline]
    fn from(src: &'a mut [Request]) -> Self {
        unsafe { transmute(src) }
    }
}
impl<'b> RequestSlice<'b> {
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
        self.1.len()
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
                self.1.len() as c_int,
                self.1.as_mut_ptr(),
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
                self.1.len() as c_int,
                self.1.as_mut_ptr(),
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
                self.1.len() as c_int,
                self.1.as_mut_ptr(),
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
pub struct RequestVec<'b>(PhantomData<&'b [u8]>, Vec<MPI_Request>);
impl<'a, 'b> From<Vec<Request<'b>>> for RequestVec<'b> {
    #[inline]
    fn from(src: Vec<Request>) -> Self {
        unsafe { transmute(src) }
    }
}
impl<'b> Deref for RequestVec<'b> {
    type Target = RequestSlice<'b>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute::<&[MPI_Request], &RequestSlice>(&*self.1) }
    }
}
impl<'b> DerefMut for RequestVec<'b> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute::<&mut [MPI_Request], &mut RequestSlice>(&mut *self.1) }
    }
}
impl<'b> RequestVec<'b> {
    #[inline]
    pub fn push(&mut self, request: Request) {
        self.1.push(request.into_raw())
    }
}

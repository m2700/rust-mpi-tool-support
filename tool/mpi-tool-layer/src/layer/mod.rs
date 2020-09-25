use std::os::raw::*;

use mpi_sys::pmpi::*;
use rmpi::pmpi_mode::{
    datatype::RawDatatype, Communicator, RmpiResult, TypeDynamicBufferMut, TypeDynamicBufferRef,
};

#[inline]
fn buffers_from_displs<'a>(
    buf: *const c_void,
    counts: *const c_int,
    displs: *const c_int,
    datatype: MPI_Datatype,
    comm: MPI_Comm,
) -> RmpiResult<Vec<TypeDynamicBufferRef<'a>>> {
    let datatype = unsafe { RawDatatype::from_raw(datatype) };
    let mut buffers = vec![];
    if !(buf.is_null() || counts.is_null() || displs.is_null()) {
        for send_rank in 0..unsafe { Communicator::from_raw_ref(&comm) }.size().unwrap() {
            let send_buffer_part = unsafe {
                TypeDynamicBufferRef::from_raw_dynamic(
                    buf.add((*displs.add(send_rank as usize) * datatype.size()?) as usize)
                        as *const c_void,
                    *counts.add(send_rank as usize),
                    datatype.as_raw(),
                )
            };
            buffers.push(send_buffer_part);
        }
    }
    Ok(buffers)
}
#[inline]
fn buffers_mut_from_displs<'a>(
    buf: *mut c_void,
    counts: *const c_int,
    displs: *const c_int,
    datatype: MPI_Datatype,
    comm: MPI_Comm,
) -> RmpiResult<Vec<TypeDynamicBufferMut<'a>>> {
    let datatype = unsafe { RawDatatype::from_raw(datatype) };
    let mut recv_buffers = vec![];
    if !(buf.is_null() || counts.is_null() || displs.is_null()) {
        for recv_rank in 0..unsafe { Communicator::from_raw_ref(&comm) }.size().unwrap() {
            let recv_buffer_part = unsafe {
                TypeDynamicBufferMut::from_raw_dynamic(
                    buf.add(
                        (*displs.add(recv_rank as usize) as usize) * (datatype.size()? as usize),
                    ) as *mut c_void,
                    *counts.add(recv_rank as usize),
                    datatype.as_raw(),
                )
            };
            recv_buffers.push(recv_buffer_part);
        }
    }
    Ok(recv_buffers)
}

mod limpl;
mod ltrait;

pub use ltrait::MpiInterceptionLayer;

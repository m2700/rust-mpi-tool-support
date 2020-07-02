use std::os::raw::{c_double, c_float, c_int, c_void};

use mpi_sys::{MPI_Comm, MPI_Datatype};
use rmpi::{Buffer, Communicator, MpiDatatype, Process, RmpiResult};

use crate::{RawMpiInterceptionLayer, UnsafeBox};

pub trait MpiInterceptionLayer {
    #[inline]
    fn send<F, Buf>(next_f: F, buf: &Buf, dest: Process, tag: c_int) -> RmpiResult
    where
        F: FnOnce(&Buf, Process, c_int) -> RmpiResult,
        Buf: rmpi::Buffer + ?Sized,
    {
        next_f(buf, dest, tag)
    }
}

macro_rules! unsafe_with_buf {
    ( ($buf:ident, $count:ident, $datatype:ident) => $buffer:pat => $tdblck:block else $eblck:block ) => {
        if $datatype == u8::datatype() {
            let $buffer = unsafe { <[u8] as Buffer>::from_raw($buf, $count) };
            $tdblck
        }
        else if $datatype == i8::datatype() {
            let $buffer = unsafe { <[i8] as Buffer>::from_raw($buf, $count) };
            $tdblck
        }
        else if $datatype == u16::datatype() {
            let $buffer = unsafe { <[u16] as Buffer>::from_raw($buf, $count) };
            $tdblck
        }
        else if $datatype == i16::datatype() {
            let $buffer = unsafe { <[i16] as Buffer>::from_raw($buf, $count) };
            $tdblck
        }
        else if $datatype == u32::datatype() {
            let $buffer = unsafe { <[u32] as Buffer>::from_raw($buf, $count) };
            $tdblck
        }
        else if $datatype == i32::datatype() {
            let $buffer = unsafe { <[i32] as Buffer>::from_raw($buf, $count) };
            $tdblck
        }
        else if $datatype == u64::datatype() {
            let $buffer = unsafe { <[u64] as Buffer>::from_raw($buf, $count) };
            $tdblck
        }
        else if $datatype == i64::datatype() {
            let $buffer = unsafe { <[i64] as Buffer>::from_raw($buf, $count) };
            $tdblck
        }
        else if $datatype == c_float::datatype() {
            let $buffer = unsafe { <[c_float] as Buffer>::from_raw($buf, $count) };
            $tdblck
        }
        else if $datatype == c_double::datatype() {
            let $buffer = unsafe { <[c_double] as Buffer>::from_raw($buf, $count) };
            $tdblck
        }
        else $eblck
    };
}

impl<T> RawMpiInterceptionLayer for T
where
    T: MpiInterceptionLayer,
{
    #[inline]
    fn send<F>(
        next_f: UnsafeBox<F>,
        buf: *const c_void,
        count: c_int,
        datatype: MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*const c_void, c_int, MPI_Datatype, c_int, c_int, MPI_Comm) -> c_int,
    {
        unsafe_with_buf!(
            (buf,count,datatype) => buffer => {
                rmpi::Error::result_into_mpi_res(
                    <Self as MpiInterceptionLayer>::send(
                        |buf, mut dest, tag| {
                            unsafe{dest.send_with(next_f.unwrap(), buf, tag)}
                        },
                        buffer, unsafe{Communicator::from_raw(comm)}.get_process(dest), tag
                    )
                )
            }
            else {
                unsafe {
                    next_f.unwrap()(buf,count,datatype,dest,tag,comm)
                }
            }
        )
    }
}

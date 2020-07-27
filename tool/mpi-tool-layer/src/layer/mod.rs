macro_rules! unsafe_with_buf {
    ( ($buf:ident, $count:ident, $datatype:ident) => $buffer:pat => $ret:expr ) => {
        define_datatype! {
            type Elem = $datatype;
            {
                let $buffer = unsafe { <[Elem] as Buffer>::from_raw($buf, $count) };
                $ret
            }
        }
    };
}
macro_rules! unsafe_with_buf_mut {
    ( ($buf:ident, $count:ident, $datatype:ident) => $buffer:pat => $ret:expr ) => {
        define_datatype! {
            type Elem = $datatype;
            {
                let $buffer = unsafe { <[Elem] as Buffer>::from_raw_mut($buf, $count) };
                $ret
            }
        }
    };
}

macro_rules! define_datatype {
    ( type $dt_tp:ident = $datatype:ident; $ret:expr ) => {{
        let datatype = $crate::layer::simplify_datatype_rust($datatype);
        if datatype == u8::datatype() || datatype == MPI_DATATYPE_NULL {
            type $dt_tp = u8;
            $ret
        } else if datatype == i8::datatype() {
            type $dt_tp = i8;
            $ret
        } else if datatype == u16::datatype() {
            type $dt_tp = u16;
            $ret
        } else if datatype == i16::datatype() {
            type $dt_tp = i16;
            $ret
        } else if datatype == u32::datatype() {
            type $dt_tp = u32;
            $ret
        } else if datatype == i32::datatype() {
            type $dt_tp = i32;
            $ret
        } else if datatype == u64::datatype() {
            type $dt_tp = u64;
            $ret
        } else if datatype == i64::datatype() {
            type $dt_tp = i64;
            $ret
        } else if datatype == c_float::datatype() {
            type $dt_tp = c_float;
            $ret
        } else if datatype == c_double::datatype() {
            type $dt_tp = c_double;
            $ret
        } else if datatype == cnum::Complex::<c_float>::datatype() {
            type $dt_tp = cnum::Complex<c_float>;
            $ret
        } else if datatype == cnum::Complex::<c_double>::datatype() {
            type $dt_tp = cnum::Complex<c_double>;
            $ret
        } else if datatype == LongInt::datatype() {
            type $dt_tp = LongInt;
            $ret
        } else if datatype == DoubleInt::datatype() {
            type $dt_tp = DoubleInt;
            $ret
        } else if datatype == ShortInt::datatype() {
            type $dt_tp = ShortInt;
            $ret
        } else if datatype == TwoInt::datatype() {
            type $dt_tp = TwoInt;
            $ret
        } else if datatype == LongDoubleInt::datatype() {
            type $dt_tp = LongDoubleInt;
            $ret
        } else {
            panic!("{:?} not supported", datatype)
        }
    }};
}

use std::os::raw::*;

use cnum::Complex;
use mpi_sys::pmpi::*;
use rmpi::pmpi_mode::{CppBool, MpiDatatype};

#[inline]
fn simplify_datatype_rust(datatype: MPI_Datatype) -> MPI_Datatype {
    match datatype {
        MPI_UINT8_T => u8::datatype(),
        MPI_UINT16_T => u16::datatype(),
        MPI_UINT32_T => u32::datatype(),
        MPI_UINT64_T => u64::datatype(),
        MPI_INT8_T => i8::datatype(),
        MPI_INT16_T => i16::datatype(),
        MPI_INT32_T => i32::datatype(),
        MPI_INT64_T => i64::datatype(),
        MPI_CHAR => c_char::datatype(),
        MPI_UNSIGNED_CHAR => c_uchar::datatype(),
        MPI_SIGNED_CHAR => c_schar::datatype(),
        MPI_SHORT => c_short::datatype(),
        MPI_UNSIGNED_SHORT => c_ushort::datatype(),
        MPI_INT => c_int::datatype(),
        MPI_UNSIGNED => c_uint::datatype(),
        MPI_LONG => c_long::datatype(),
        MPI_UNSIGNED_LONG => c_ulong::datatype(),
        MPI_LONG_LONG_INT => c_longlong::datatype(),
        MPI_FLOAT => c_float::datatype(),
        MPI_DOUBLE => c_double::datatype(),
        MPI_C_BOOL => CppBool::datatype(),
        MPI_C_FLOAT_COMPLEX => Complex::<c_float>::datatype(),
        MPI_C_DOUBLE_COMPLEX => Complex::<c_double>::datatype(),
        MPI_C_LONG_DOUBLE_COMPLEX => {
            panic!("rust does not support the datatype long double complex")
        }
        MPI_LONG_INT => LongInt::datatype(),
        MPI_DOUBLE_INT => DoubleInt::datatype(),
        MPI_SHORT_INT => ShortInt::datatype(),
        MPI_2INT => TwoInt::datatype(),
        MPI_LONG_DOUBLE_INT => LongDoubleInt::datatype(),
        _ => panic!("{:?} not supported", datatype),
    }
}

mod limpl;
mod ltrait;

pub use ltrait::MpiInterceptionLayer;

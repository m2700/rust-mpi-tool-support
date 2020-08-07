macro_rules! unsafe_with_buf {
    ( ($buf:ident, $count:ident, $datatype:ident) => $buffer:pat => $ret:expr ) => {
        define_datatype! {
            type Elem = $datatype;
            {
                let $buffer = unsafe { <[Elem] as Buffer>::from_raw($buf, $count) };
                $ret
            }
            dynamic {
                let $buffer = unsafe {
                    <TypeDynamicBufferRef as BufferRef>::from_raw2($buf, $count, $datatype)
                };
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
            dynamic {
                let $buffer = unsafe {
                    <TypeDynamicBufferMut as BufferMut>::from_raw_mut2($buf, $count, $datatype)
                };
                $ret
            }
        }
    };
}

// macro_rules! define_buffertype {
//     ( type $bt_tp:ident = &[$datatype:ident]; $ret:expr ) => (
//         define_datatype!(
//             type Elem = $datatype;
//             {
//                 type $bt_tp = &[Elem];
//                 $ret
//             }
//             dynamic {
//                 type $bt_tp = TypeDynamicBufferRef;
//                 $ret
//             }
//         )
//     );
//     ( type $bt_tp:ident = &mut [$datatype:ident]; $ret:expr ) => (
//         define_datatype!(
//             type Elem = $datatype;
//             {
//                 type $bt_tp = &mut [Elem];
//                 $ret
//             }
//             dynamic {
//                 type $bt_tp = TypeDynamicBufferMut;
//                 $ret
//             }
//         )
//     );
// }

macro_rules! define_datatype {
    ( type $dt_tp:ident = $datatype:ident; $ret:expr ) => (
        define_datatype!(
            type $dt_tp = $datatype;
            { $ret }
            dynamic {
                panic!("{:?} not supported", $datatype)
            }
        )
    );
    ( type $dt_tp:ident = $datatype:ident; $ret:block dynamic $dynret:block ) => {{
        let datatype = $crate::layer::simplify_datatype_rust($datatype);
        if datatype == u8::datatype().as_raw() || datatype == MPI_DATATYPE_NULL {
            type $dt_tp = u8;
            $ret
        } else if datatype == i8::datatype().as_raw() {
            type $dt_tp = i8;
            $ret
        } else if datatype == u16::datatype().as_raw() {
            type $dt_tp = u16;
            $ret
        } else if datatype == i16::datatype().as_raw() {
            type $dt_tp = i16;
            $ret
        } else if datatype == u32::datatype().as_raw() {
            type $dt_tp = u32;
            $ret
        } else if datatype == i32::datatype().as_raw() {
            type $dt_tp = i32;
            $ret
        } else if datatype == u64::datatype().as_raw() {
            type $dt_tp = u64;
            $ret
        } else if datatype == i64::datatype().as_raw() {
            type $dt_tp = i64;
            $ret
        } else if datatype == c_float::datatype().as_raw() {
            type $dt_tp = c_float;
            $ret
        } else if datatype == c_double::datatype().as_raw() {
            type $dt_tp = c_double;
            $ret
        } else if datatype == cnum::Complex::<c_float>::datatype().as_raw() {
            type $dt_tp = cnum::Complex<c_float>;
            $ret
        } else if datatype == cnum::Complex::<c_double>::datatype().as_raw() {
            type $dt_tp = cnum::Complex<c_double>;
            $ret
        } else if datatype == LongInt::datatype().as_raw() {
            type $dt_tp = LongInt;
            $ret
        } else if datatype == DoubleInt::datatype().as_raw() {
            type $dt_tp = DoubleInt;
            $ret
        } else if datatype == ShortInt::datatype().as_raw() {
            type $dt_tp = ShortInt;
            $ret
        } else if datatype == TwoInt::datatype().as_raw() {
            type $dt_tp = TwoInt;
            $ret
        } else if datatype == LongDoubleInt::datatype().as_raw() {
            type $dt_tp = LongDoubleInt;
            $ret
        } else $dynret
    }};
}

use std::os::raw::*;

use cnum::Complex;
use mpi_sys::pmpi::*;
use rmpi::pmpi_mode::datatype::{CppBool, MpiPredefinedDatatype};

#[inline]
fn simplify_datatype_rust(datatype: MPI_Datatype) -> MPI_Datatype {
    match datatype {
        MPI_UINT8_T => u8::datatype().as_raw(),
        MPI_UINT16_T => u16::datatype().as_raw(),
        MPI_UINT32_T => u32::datatype().as_raw(),
        MPI_UINT64_T => u64::datatype().as_raw(),
        MPI_INT8_T => i8::datatype().as_raw(),
        MPI_INT16_T => i16::datatype().as_raw(),
        MPI_INT32_T => i32::datatype().as_raw(),
        MPI_INT64_T => i64::datatype().as_raw(),
        MPI_CHAR => c_char::datatype().as_raw(),
        MPI_UNSIGNED_CHAR => c_uchar::datatype().as_raw(),
        MPI_SIGNED_CHAR => c_schar::datatype().as_raw(),
        MPI_SHORT => c_short::datatype().as_raw(),
        MPI_UNSIGNED_SHORT => c_ushort::datatype().as_raw(),
        MPI_INT => c_int::datatype().as_raw(),
        MPI_UNSIGNED => c_uint::datatype().as_raw(),
        MPI_LONG => c_long::datatype().as_raw(),
        MPI_UNSIGNED_LONG => c_ulong::datatype().as_raw(),
        MPI_LONG_LONG_INT => c_longlong::datatype().as_raw(),
        MPI_FLOAT => c_float::datatype().as_raw(),
        MPI_DOUBLE => c_double::datatype().as_raw(),
        MPI_C_BOOL => CppBool::datatype().as_raw(),
        MPI_C_FLOAT_COMPLEX => Complex::<c_float>::datatype().as_raw(),
        MPI_C_DOUBLE_COMPLEX => Complex::<c_double>::datatype().as_raw(),
        // MPI_C_LONG_DOUBLE_COMPLEX => {
        //     panic!("rust does not support the datatype long double complex")
        // }
        MPI_LONG_INT => LongInt::datatype().as_raw(),
        MPI_DOUBLE_INT => DoubleInt::datatype().as_raw(),
        MPI_SHORT_INT => ShortInt::datatype().as_raw(),
        MPI_2INT => TwoInt::datatype().as_raw(),
        MPI_LONG_DOUBLE_INT => LongDoubleInt::datatype().as_raw(),
        dtt => dtt,
    }
}

mod limpl;
mod ltrait;

pub use ltrait::MpiInterceptionLayer;

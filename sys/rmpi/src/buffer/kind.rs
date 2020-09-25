use std::os::raw::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ushort,
};

use cnum::Complex;

local_mod!(
    use mpi_sys::*;
    use crate::{
        datatype::{MpiPredefinedDatatype, CppBool},
    };
);

use super::*;

macro_rules! define_datatype {
    ( type $dt_tp:ident = $datatype:expr; $ret:block dynamic $dynret:block ) => {{
        let datatype = simplify_datatype_rust($datatype);
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

#[inline]
pub(super) fn simplify_datatype_rust(datatype: MPI_Datatype) -> MPI_Datatype {
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
        // rust does not support the datatype long double complex
        // }
        MPI_LONG_INT => LongInt::datatype().as_raw(),
        MPI_DOUBLE_INT => DoubleInt::datatype().as_raw(),
        MPI_SHORT_INT => ShortInt::datatype().as_raw(),
        MPI_2INT => TwoInt::datatype().as_raw(),
        MPI_LONG_DOUBLE_INT => LongDoubleInt::datatype().as_raw(),
        dtt => dtt,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BufferRefKind<'a> {
    U8(&'a [u8]),
    U16(&'a [u16]),
    U32(&'a [u32]),
    U64(&'a [u64]),
    I8(&'a [i8]),
    I16(&'a [i16]),
    I32(&'a [i32]),
    I64(&'a [i64]),
    Float(&'a [c_float]),
    Double(&'a [c_double]),
    CppBool(&'a [CppBool]),
    ComplexFloat(&'a [Complex<c_float>]),
    ComplexDouble(&'a [Complex<c_double>]),
    LongInt(&'a [LongInt]),
    DoubleInt(&'a [DoubleInt]),
    ShortInt(&'a [ShortInt]),
    TwoInt(&'a [TwoInt]),
    LongDoubleInt(&'a [LongDoubleInt]),
    TypeDynamic(TypeDynamicBufferRef<'a>),
}
#[derive(Debug)]
pub enum BufferMutKind<'a> {
    U8(&'a mut [u8]),
    U16(&'a mut [u16]),
    U32(&'a mut [u32]),
    U64(&'a mut [u64]),
    I8(&'a mut [i8]),
    I16(&'a mut [i16]),
    I32(&'a mut [i32]),
    I64(&'a mut [i64]),
    Float(&'a mut [c_float]),
    Double(&'a mut [c_double]),
    CppBool(&'a mut [CppBool]),
    ComplexFloat(&'a mut [Complex<c_float>]),
    ComplexDouble(&'a mut [Complex<c_double>]),
    LongInt(&'a mut [LongInt]),
    DoubleInt(&'a mut [DoubleInt]),
    ShortInt(&'a mut [ShortInt]),
    TwoInt(&'a mut [TwoInt]),
    LongDoubleInt(&'a mut [LongDoubleInt]),
    TypeDynamic(TypeDynamicBufferMut<'a>),
}

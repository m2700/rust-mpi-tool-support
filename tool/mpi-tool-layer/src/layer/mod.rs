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
    ( type $dt_tp:ident = $datatype:ident; $ret:expr ) => {
        if $datatype == u8::datatype() {
            type $dt_tp = u8;
            $ret
        } else if $datatype == i8::datatype() {
            type $dt_tp = i8;
            $ret
        } else if $datatype == u16::datatype() {
            type $dt_tp = u16;
            $ret
        } else if $datatype == i16::datatype() {
            type $dt_tp = i16;
            $ret
        } else if $datatype == u32::datatype() {
            type $dt_tp = u32;
            $ret
        } else if $datatype == i32::datatype() {
            type $dt_tp = i32;
            $ret
        } else if $datatype == u64::datatype() {
            type $dt_tp = u64;
            $ret
        } else if $datatype == i64::datatype() {
            type $dt_tp = i64;
            $ret
        } else if $datatype == c_float::datatype() {
            type $dt_tp = c_float;
            $ret
        } else if $datatype == c_double::datatype() {
            type $dt_tp = c_double;
            $ret
        } else if $datatype == MPI_CHAR {
            type $dt_tp = ::std::os::raw::c_char;
            $ret
        } else if $datatype == MPI_UNSIGNED_CHAR {
            type $dt_tp = ::std::os::raw::c_uchar;
            $ret
        } else if $datatype == MPI_SIGNED_CHAR {
            type $dt_tp = ::std::os::raw::c_schar;
            $ret
        } else if $datatype == MPI_SHORT {
            type $dt_tp = ::std::os::raw::c_short;
            $ret
        } else if $datatype == MPI_UNSIGNED_SHORT {
            type $dt_tp = ::std::os::raw::c_ushort;
            $ret
        } else if $datatype == MPI_INT {
            type $dt_tp = ::std::os::raw::c_int;
            $ret
        } else if $datatype == MPI_UNSIGNED {
            type $dt_tp = ::std::os::raw::c_uint;
            $ret
        } else if $datatype == MPI_LONG {
            type $dt_tp = ::std::os::raw::c_long;
            $ret
        } else if $datatype == MPI_UNSIGNED_LONG {
            type $dt_tp = ::std::os::raw::c_ulong;
            $ret
        } else if $datatype == MPI_LONG_LONG_INT {
            type $dt_tp = ::std::os::raw::c_longlong;
            $ret
        } else {
            panic!("{:?} not supported", $datatype)
        }
    };
}

mod limpl;
mod ltrait;

pub use ltrait::MpiInterceptionLayer;

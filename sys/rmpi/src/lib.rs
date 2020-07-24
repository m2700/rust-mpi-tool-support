macro_rules! local_mod_decide {
    (
        crate:: $($ptt:tt)*
    ) => (
        use crate:: $($ptt)* ;
    );
    (
        mpi_sys:: $($ptt:tt)*
    ) => (
        use mpi_sys:: $($ptt)* ;
    );
}

macro_rules! local_mod {
    () => ();
    (
        use $( $pth_prt_id:ident )::*;
        $($cnt:tt)*
    ) => {
        local_mod_decide!($( $pth_prt_id )::*);
        local_mod!($($cnt)*);
    };
    (
        use $( $pth_prt_id:ident :: )* *;
        $($cnt:tt)*
    ) => {
        local_mod_decide!($( $pth_prt_id :: )* *);
        local_mod!($($cnt)*);
    };
    (
        use $( $pth_prt_id:ident :: )* { $($tkn:tt)* };
        $($cnt:tt)*
    ) => {
        local_mod_decide!($( $pth_prt_id :: )* { $($tkn)* });
        local_mod!($($cnt)*);
    };
}
include!("lib_mode_dependent.rs");

#[cfg(feature = "tool_mode")]
pub mod pmpi_mode {
    macro_rules! local_mod_decide {
        (
            crate:: $($ptt:tt)*
        ) => (
            use crate::pmpi_mode:: $($ptt)* ;
        );
        (
            mpi_sys:: $($ptt:tt)*
        ) => (
            use mpi_sys::pmpi:: $($ptt)* ;
        );
    }
    include!("lib_mode_dependent.rs");
}

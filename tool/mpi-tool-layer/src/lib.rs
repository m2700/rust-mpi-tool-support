mod raw_layer;
mod unsafe_box;

pub use raw_layer::RawMpiInterceptionLayer;
pub use unsafe_box::UnsafeBox;

#[cfg(feature = "rmpi_support")]
macro_rules! trait_layer_function {
    (
        $( #[ $mt:meta ] )*
        fn $fn_id:ident
        $(< $($gen_lft:lifetime),* $(,)? $( $gen_tp:ident $(: $lft_cns:lifetime )? ),* >)?
        ( $nextf:ident : UnsafeBox $(, $argp:ident : $argt:ty)* $(,)? ) -> $ret:ty
            $(
                where $($cns_tp:ty : $tr_cns:ident $( + $adt_tr_cns:ident)* ),*
            )?
        $blck:block

        $( $($nxt:tt)+ )?
    ) => {
        $( #[ $mt ] )*
        fn $fn_id < $($gen_lft ,)* F $(, $( $gen_tp $(: $lft_cns)? ),*)? >
        ( $nextf: UnsafeBox<F>, $($argp : $argt),* ) -> $ret
            where
                F: FnOnce($($argt),*) -> $ret,
                $(
                    $($cns_tp : $tr_cns $( + $adt_tr_cns)* ),*
                )?
        $blck

        $( trait_layer_function!($($nxt)+); )?
    };
    (
        $( #[ $mt:meta ] )*
        fn $fn_id:ident
        $(< $($gen_lft:lifetime),* $(,)? $( $gen_tp:ident $(: $lft_cns:lifetime)? ),* >)?
        ( $nextf:ident $(, $argp:ident : $argt:ty)* $(,)? ) -> $ret:ty
            $(
                where $($cns_tp:ty : $tr_cns:ident $( + $adt_tr_cns:ident)* ),*
            )?
        $blck:block

        $( $($nxt:tt)+ )?
    ) => {
        $( #[ $mt ] )*
        fn $fn_id < $($($gen_lft ,)*)? F $(, $( $gen_tp $(: $lft_cns)? ),*)? >
        ( $nextf: F, $($argp : $argt),* ) -> $ret
            where
                F: FnOnce($($argt),*) -> $ret,
                $(
                    $($cns_tp : $tr_cns $( + $adt_tr_cns)* ),*
                )?
        $blck

        $( trait_layer_function!($($nxt)+); )?
    };
    (
        $( #[ $mt:meta ] )*
        unsafe fn $fn_id:ident
        $(< $($gen_lft:lifetime),* $(,)? $( $gen_tp:ident $(: $lft_cns:lifetime)? ),* >)?
        ( $nextf:ident $(, $argp:ident : $argt:ty)* $(,)? ) -> $ret:ty
            $(
                where $($cns_tp:ty : $tr_cns:ident $( + $adt_tr_cns:ident)* ),*
            )?
        $blck:block

        $( $($nxt:tt)+ )?
    ) => {
        $( #[ $mt ] )*
        unsafe fn $fn_id < $($($gen_lft ,)*)? F $(, $( $gen_tp $(: $lft_cns)? ),*)? >
        ( $nextf: F, $($argp : $argt),* ) -> $ret
            where
                F: FnOnce($($argt),*) -> $ret,
                $(
                    $($cns_tp : $tr_cns $( + $adt_tr_cns)* ),*
                )?
        $blck

        $( trait_layer_function!($($nxt)+); )?
    };

    (
        $( #[ $mt:meta ] )*
        fn $fn_id:ident
        $(< $($gen_lft:lifetime),* $(,)? $( $gen_tp:ident $(: $lft_cns:lifetime)? ),* >)?
        ( UnsafeBox $(, $argp:ident : $argt:ty)* $(,)? ) -> $ret:ty
            $(
                where $($cns_tp:ty : $tr_cns:ident $( + $adt_tr_cns:ident)* ),*
            )? ;

        $( $($nxt:tt)+ )?
    ) => {
        trait_layer_function!{
            $( #[ $mt ] )*
            fn $fn_id $(< $($gen_lft ,)* $( $gen_tp $(: $lft_cns)? ),* >)?
            ( next_f: UnsafeBox, $($argp : $argt),* ) -> $ret
                $(
                    where $($cns_tp : $tr_cns $( + $adt_tr_cns)* ),*
                )?
            {
                unsafe{next_f.unwrap()( $($argp),* )}
            }

            $( $($nxt)+ )?
        }
    };
    (
        $( #[ $mt:meta ] )*
        fn $fn_id:ident
        $(< $($gen_lft:lifetime),* $(,)? $( $gen_tp:ident $(: $lft_cns:lifetime)? ),* >)?
        ( $($argp:ident : $argt:ty),* $(,)? ) -> $ret:ty
            $(
                where $($cns_tp:ty : $tr_cns:ident $( + $adt_tr_cns:ident)* ),*
            )? ;

        $( $($nxt:tt)+ )?
    ) => {
        trait_layer_function!{
            $( #[ $mt ] )*
            fn $fn_id $(< $($gen_lft ,)* $( $gen_tp $(: $lft_cns)? ),* >)?
            ( next_f, $($argp : $argt),* ) -> $ret
                $(
                    where $($cns_tp : $tr_cns $( + $adt_tr_cns)* ),*
                )?
            {
                next_f( $($argp),* )
            }

            $( $($nxt)+ )?
        }
    };
    (
        $( #[ $mt:meta ] )*
        unsafe fn $fn_id:ident
        $(< $($gen_lft:lifetime),* $(,)? $( $gen_tp:ident $(: $lft_cns:lifetime)? ),* >)?
        ( $($argp:ident : $argt:ty),* $(,)? ) -> $ret:ty
            $(
                where $($cns_tp:ty : $tr_cns:ident $( + $adt_tr_cns:ident)* ),*
            )? ;

        $( $($nxt:tt)+ )?
    ) => {
        trait_layer_function!{
            $( #[ $mt ] )*
            unsafe fn $fn_id $(< $($gen_lft ,)* $( $gen_tp $(: $lft_cns)? ),* >)?
            ( next_f, $($argp : $argt),* ) -> $ret
                $(
                    where $($cns_tp : $tr_cns $( + $adt_tr_cns)* ),*
                )?
            {
                next_f( $($argp),* )
            }

            $( $($nxt)+ )?
        }
    };
}

pub use mpi_sys::pmpi as mpi_sys;

#[cfg(feature = "rmpi_support")]
mod layer;
#[cfg(feature = "rmpi_support")]
pub use layer::MpiInterceptionLayer;
#[cfg(feature = "rmpi_support")]
pub use rmpi::pmpi_mode as rmpi;

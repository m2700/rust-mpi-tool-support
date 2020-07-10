mod raw_layer;

pub use raw_layer::{RawMpiInterceptionLayer, UnsafeBox};

macro_rules! trait_layer_function {
    (
        $( #[ $mt:meta ] )*
        fn $fn_id:ident $(< $( $gen_tp:ident $(: ?$mrk_cns:ident)? ),* >)?
        ( unsafe $nextf:ident $(, $argp:ident : $argt:ty)* $(,)? ) -> $ret:ty
            $(
                where $($cns_tp:ty : $tr_cns:ident $( + $adt_tr_cns:ident)* ),*
            )?
        $blck:block

        $( $($nxt:tt)+ )?
    ) => {
        $( #[ $mt ] )*
        fn $fn_id < F $(, $( $gen_tp $(: ?$mrk_cns)? ),*)? >
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
        fn $fn_id:ident $(< $( $gen_tp:ident $(: ?$mrk_cns:ident)? ),* >)?
        ( $nextf:ident $(, $argp:ident : $argt:ty)* $(,)? ) -> $ret:ty
            $(
                where $($cns_tp:ty : $tr_cns:ident $( + $adt_tr_cns:ident)* ),*
            )?
        $blck:block

        $( $($nxt:tt)+ )?
    ) => {
        $( #[ $mt ] )*
        fn $fn_id < F $(, $( $gen_tp $(: ?$mrk_cns)? ),*)? >
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
        fn $fn_id:ident $(< $( $gen_tp:ident $(: ?$mrk_cns:ident)? ),* >)?
        ( unsafe $(, $argp:ident : $argt:ty)* $(,)? ) -> $ret:ty
            $(
                where $($cns_tp:ty : $tr_cns:ident $( + $adt_tr_cns:ident)* ),*
            )? ;

        $( $($nxt:tt)+ )?
    ) => {
        trait_layer_function!{
            $( #[ $mt ] )*
            fn $fn_id $(< $( $gen_tp $(: ?$mrk_cns)? ),* >)?
            ( unsafe next_f, $($argp : $argt),* ) -> $ret
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
        fn $fn_id:ident $(< $( $gen_tp:ident $(: ?$mrk_cns:ident)? ),* >)?
        ( $($argp:ident : $argt:ty),* $(,)? ) -> $ret:ty
            $(
                where $($cns_tp:ty : $tr_cns:ident $( + $adt_tr_cns:ident)* ),*
            )? ;

        $( $($nxt:tt)+ )?
    ) => {
        trait_layer_function!{
            $( #[ $mt ] )*
            fn $fn_id $(< $( $gen_tp $(: ?$mrk_cns)? ),* >)?
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

#[cfg(feature = "rmpi_support")]
mod layer;
#[cfg(feature = "rmpi_support")]
pub use layer::MpiInterceptionLayer;

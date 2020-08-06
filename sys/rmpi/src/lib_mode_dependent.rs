macro_rules! tool_mode_item {
    (
        $( #[ $m:meta ] )*
        pub $($fntkns:tt)*
    ) => {
        #[cfg(feature = "tool_mode")]
        $( #[ $m ] )*
        pub $($fntkns)*

        #[cfg(not(feature = "tool_mode"))]
        $( #[ $m ] )*
        pub(crate) $($fntkns)*
    };
}

mod buffer;
mod communicator;
mod context;
pub mod datatype;
mod error;
mod group;
mod mpi_op;
mod process;
pub mod request;
mod status;
mod tag;

pub use self::{
    buffer::*, communicator::*, context::*, error::*, group::*, mpi_op::*, process::*, status::*,
    tag::*,
};

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
mod error;
mod process;
pub mod request;
mod status;
mod tag;

pub use buffer::{Buffer, MpiDatatype};
pub use communicator::Communicator;
pub use error::{Error, RmpiResult};
pub use process::Process;
pub use status::Status;
pub use tag::Tag;

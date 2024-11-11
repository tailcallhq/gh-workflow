mod cargo;
pub mod error;
mod event;
pub mod generate;
pub mod release_plz;
mod rust_flag;
pub mod toolchain;
pub(crate) mod workflow;

pub use cargo::*;
pub use event::*;
pub use rust_flag::*;
pub use workflow::*;

mod private {
    pub trait Sealed {}
}

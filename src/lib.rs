#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations, nonstandard_style)]
#![warn(unreachable_pub, rust_2018_idioms)]

pub use miette_derive::*;

pub use context::*;
pub use error::*;
pub use named_source::*;
pub use printer::*;
pub use protocol::*;
pub use deprecated::*;

mod chain;
mod context;
mod error;
mod named_source;
mod printer;
mod protocol;
mod source_impls;
mod deprecated;

#![forbid(unsafe_code)]

#[macro_use]
#[path = "generated/decompose.rs"]
mod decompose;

#[doc(hidden)]
pub mod macros {
	pub use super::decompose::*;
	pub use core::option::Option::{None, Some};
}

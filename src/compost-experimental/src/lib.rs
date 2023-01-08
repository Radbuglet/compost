#![no_std]
#![forbid(unsafe_code)]

mod decompose;

#[doc(hidden)]
pub mod macro_internal {
    pub use super::decompose::*;
    pub use core::{
        convert::identity,
        option::Option::{None, Some},
    };
}

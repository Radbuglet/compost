//! # Overview
//!
//! This library exposes [`decompose!`](crate::decompose), a macro to decompose tuples into tuples
//! containing a subset of their values.
//!
//! ```
//! use compost::decompose;
//!
//! // Pack all your context into a tuple...
//! let mut cx = (&mut 1u32, &2u8, (&3u16, &mut 'e', "f", &mut [1, 2, 3]));
//!
//! // And cal your target function!
//! consumer(decompose!(cx));
//!
//! // Define functions taking tuples of context...
//! fn consumer_2((a, b, c, d, e): (&u32, &str, &char, &f32, &i8)) {
//!     dbg!(a, b, c, d, e);
//! }
//!
//! fn consumer_3((f, g, h): (&char, &str, &mut [u8; 3])) {
//!     dbg!(f, g, h);
//! }
//!
//! fn consumer(mut cx: (&mut u32, &char, &str, &mut [u8; 3])) {
//!     // Bring types into scope and produce a remainder tuple.
//!     decompose!(cx => rest & {
//!         char: &char,
//!         bytes: &mut [u8; 3],
//!     });
//!
//!     // Combine contexts...
//!     let mut rest = (rest, (&mut 4.3f32, &-4i8), &mut 5i16);
//!
//!     // ...and forward them to further functions.
//!     consumer_2(decompose!(rest));
//!
//!     // ...all without invalidating existing borrows!
//!     dbg!(char, bytes);
//!
//!     // Reborrow the original context after its borrows have expired.
//!     consumer_3(decompose!(cx));
//! }
//! ```
//!
//! See [`decompose!`]'s documentation for more details on the precise semantics and
//! limitations of the macro.
//!
//! ## Features
//!
//! Yes, this library...
//!
//! - Supports reborrowing (i.e. `decompose!` does not consume its input. Once you're done
//!    with the borrow, you can reuse the original tuple).
//! - Produces (admittedly pretty ugly) errors at compile time if the tuple cannot be decomposed.
//! - Supports borrowing mutable, immutable, and smart-pointer wrapped (so long as they implement
//!   [`Deref`](::core::ops::Deref)) components.
//! - Supports borrowing from nested tuple trees, allowing you to borrow from an arbitrary number of
//!   components simultaneously and to quickly merge multiple context tuples by packing them into a
//!   single tuple.
//! - Supports borrowing generic elements without failing spuriously on monomorphization.
//! - Relies on type inference rather than `TypeId`, allowing the macro to operate on non-`'static` types.
//! - Supports `no_std` environments and does not rely on unsafe code.
//! - Has zero runtime dependencies.
//!

#![no_std]
#![forbid(unsafe_code)]

mod decompose;

#[doc(hidden)]
pub mod macro_internal {
    pub use super::decompose::*;
    pub use core::option::Option::{None, Some};
}

#![forbid(unsafe_code)]

#[macro_use]
#[path = "generated/decompose.rs"]
mod decompose;

#[doc(hidden)]
pub mod macros {
	pub use super::decompose::*;
	pub use core::option::Option::{None, Some};
}

#[allow(dead_code)]
mod example {
	use std::{
		borrow::{Borrow, BorrowMut},
		cell::RefMut,
	};

	struct MyRefMut<'a, T: ?Sized>(RefMut<'a, T>);

	impl<'a, T: ?Sized> Borrow<T> for MyRefMut<'a, T> {
		fn borrow(&self) -> &T {
			&self.0
		}
	}

	impl<'a, T: ?Sized> BorrowMut<T> for MyRefMut<'a, T> {
		fn borrow_mut(&mut self) -> &mut T {
			&mut self.0
		}
	}

	fn example(mut a: (MyRefMut<u32>, &mut i32, char)) {
		example_2(decompose!(a));
	}

	fn example_2(_a: (&u32, &i32, &char)) {}
}

use std::{
	borrow::{Borrow, BorrowMut},
	cell::{RefCell, RefMut},
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

fn main() {
	let cell = RefCell::new(3);

	example((
		MyRefMut(cell.borrow_mut()),
		&mut 3,
		'w',
		"whee",
		&mut 'k',
		"waz",
	))
}

fn example<T, V>(mut a: (MyRefMut<u32>, &mut i32, char, &str, &mut T, V)) {
	example_2(decompose::decompose!(a));
}

fn example_2(a: (&u32, &i32, &char)) {
	dbg!(a);
}

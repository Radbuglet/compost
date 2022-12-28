use std::{
	borrow::{Borrow, BorrowMut},
	cell::{RefCell, RefMut},
	fmt,
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

fn example<T, V: fmt::Debug>(mut a: (MyRefMut<u32>, &mut i32, char, &str, &mut T, V)) {
	example_2(compost::decompose!(a));

	compost::decompose!(a => b & { v0: &char });
	compost::decompose!(b => { v1: &str, v2: &mut V });

	dbg!((v0, v1, v2));
}

fn example_2(a: (&u32, &i32, &char)) {
	dbg!(a);
}

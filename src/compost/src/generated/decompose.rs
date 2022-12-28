use core::{borrow::{Borrow, BorrowMut}, marker::PhantomData};

// === Tuple Expansion === //

#[doc(hidden)]
pub struct TupleInputHole {
	_private: (),
}

impl TupleInputHole {
	pub fn new_mut<'a>() -> &'a mut Self {
		Box::leak(Box::new(TupleInputHole { _private: () }))
	}
}

// First, we define a mechanism for expanding all input tuples to tuples of the same arity.
#[doc(hidden)]
pub trait ArityExpand<'a> {
	type Target;

	fn normalize_arity(&'a mut self) -> Self::Target;
}

impl<'a> ArityExpand<'a> for () {
	type Target = (&'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a> ArityExpand<'a> for (P0,) {
	type Target = (&'a mut P0, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a> ArityExpand<'a> for (P0, P1) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a, P2: 'a> ArityExpand<'a> for (P0, P1, P2) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, &mut self.2, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a> ArityExpand<'a> for (P0, P1, P2, P3) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, &mut self.2, &mut self.3, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a> ArityExpand<'a> for (P0, P1, P2, P3, P4) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a> ArityExpand<'a> for (P0, P1, P2, P3, P4, P5) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a> ArityExpand<'a> for (P0, P1, P2, P3, P4, P5, P6) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a> ArityExpand<'a> for (P0, P1, P2, P3, P4, P5, P6, P7) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a, P8: 'a> ArityExpand<'a> for (P0, P1, P2, P3, P4, P5, P6, P7, P8) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a, P8: 'a, P9: 'a> ArityExpand<'a> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut TupleInputHole, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, TupleInputHole::new_mut(), TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a, P8: 'a, P9: 'a, P10: 'a> ArityExpand<'a> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut TupleInputHole);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, TupleInputHole::new_mut())
	}
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a, P8: 'a, P9: 'a, P10: 'a, P11: 'a> ArityExpand<'a> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11);

	fn normalize_arity(&'a mut self) -> Self::Target {
		(&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11)
	}
}

// === TupleBuilder === //

#[doc(hidden)]
pub struct TupleOutputHole {
	_private: (),
}

// Next, we define a `TupleBuilder.
//
// First, by calling `inference_helper()` and pretending to return this as the expression value on a
// match with `Some(T)` (`inference_helper` always returns `None`), the `decompose!` macro can infer
// the type of the tuple being returned—type `T`.
//
// For a given `TupleBuilder<(P0, P1, ...)>`, the `id()` method takes a parameter of type `P0`, returns
// that value, and produces a new builder of type `TupleBuilder<(P1, P2, ...)>`. Thus, by calling
// `id(T)` repeatedly, we can force the inference engine to infer the type of the expression passed to the
// method.
//
// In the case of a `TupleBuilder<()>`, `id` will default to expecting a value of type `TupleOutputHole`.
#[doc(hidden)]
pub struct TupleBuilder<T> {
	_ty: PhantomData<T>,
}

impl<T> TupleBuilder<T> {
	pub fn new() -> Self {
		Self { _ty: PhantomData, }
	}

	pub fn inference_helper(&self) -> Option<T> {
		None
	}
}

impl TupleBuilder<()> {
	pub fn id(self, v: TupleOutputHole) -> (TupleOutputHole, Self) {
		(v, self)
	}
}
impl<P0,> TupleBuilder<(P0,)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<()>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1> TupleBuilder<(P0, P1)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1,)>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1, P2> TupleBuilder<(P0, P1, P2)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1, P2)>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1, P2, P3> TupleBuilder<(P0, P1, P2, P3)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1, P2, P3)>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1, P2, P3, P4> TupleBuilder<(P0, P1, P2, P3, P4)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1, P2, P3, P4)>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1, P2, P3, P4, P5> TupleBuilder<(P0, P1, P2, P3, P4, P5)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1, P2, P3, P4, P5)>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1, P2, P3, P4, P5, P6> TupleBuilder<(P0, P1, P2, P3, P4, P5, P6)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1, P2, P3, P4, P5, P6)>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7> TupleBuilder<(P0, P1, P2, P3, P4, P5, P6, P7)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1, P2, P3, P4, P5, P6, P7)>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> TupleBuilder<(P0, P1, P2, P3, P4, P5, P6, P7, P8)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1, P2, P3, P4, P5, P6, P7, P8)>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> TupleBuilder<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1, P2, P3, P4, P5, P6, P7, P8, P9)>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> TupleBuilder<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)>) {
		(v, TupleBuilder::new())
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleBuilder<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)> {
	pub fn id(self, v: P0) -> (P0, TupleBuilder<(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)>) {
		(v, TupleBuilder::new())
	}
}

// === TupleSearch === //

// Now, we define a way to search a tuple of arity `MAX_ARITY`.
pub trait TupleDecompose<T, R, V> {
	fn search(self) -> (T, R);
}

impl<T> TupleDecompose<TupleOutputHole, Self, ()> for T {
	fn search(self) -> (TupleOutputHole, Self) {
		(TupleOutputHole { _private: () }, self)
	}
}

pub struct Disambiguator0 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut TupleInputHole, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator0> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P0: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut TupleInputHole, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.0).borrow(), (TupleInputHole::new_mut(), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut TupleInputHole, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator0> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P0: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut TupleInputHole, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.0).borrow_mut(), (TupleInputHole::new_mut(), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

pub struct Disambiguator1 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut TupleInputHole, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator1> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P1: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut TupleInputHole, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.1).borrow(), (self.0, TupleInputHole::new_mut(), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut TupleInputHole, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator1> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P1: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut TupleInputHole, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.1).borrow_mut(), (self.0, TupleInputHole::new_mut(), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

pub struct Disambiguator2 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut P1, &'a mut TupleInputHole, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator2> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P2: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut TupleInputHole, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.2).borrow(), (self.0, self.1, TupleInputHole::new_mut(), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut TupleInputHole, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator2> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P2: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut TupleInputHole, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.2).borrow_mut(), (self.0, self.1, TupleInputHole::new_mut(), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

pub struct Disambiguator3 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut TupleInputHole, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator3> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P3: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut TupleInputHole, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.3).borrow(), (self.0, self.1, self.2, TupleInputHole::new_mut(), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut TupleInputHole, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator3> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P3: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut TupleInputHole, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.3).borrow_mut(), (self.0, self.1, self.2, TupleInputHole::new_mut(), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

pub struct Disambiguator4 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut TupleInputHole, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator4> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P4: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut TupleInputHole, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.4).borrow(), (self.0, self.1, self.2, self.3, TupleInputHole::new_mut(), self.5, self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut TupleInputHole, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator4> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P4: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut TupleInputHole, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.4).borrow_mut(), (self.0, self.1, self.2, self.3, TupleInputHole::new_mut(), self.5, self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

pub struct Disambiguator5 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut TupleInputHole, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator5> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P5: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut TupleInputHole, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.5).borrow(), (self.0, self.1, self.2, self.3, self.4, TupleInputHole::new_mut(), self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut TupleInputHole, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator5> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P5: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut TupleInputHole, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.5).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, TupleInputHole::new_mut(), self.6, self.7, self.8, self.9, self.10, self.11))
	}
}

pub struct Disambiguator6 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut TupleInputHole, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator6> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P6: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut TupleInputHole, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.6).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, TupleInputHole::new_mut(), self.7, self.8, self.9, self.10, self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut TupleInputHole, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator6> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P6: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut TupleInputHole, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.6).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, TupleInputHole::new_mut(), self.7, self.8, self.9, self.10, self.11))
	}
}

pub struct Disambiguator7 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut TupleInputHole, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator7> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P7: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut TupleInputHole, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.7).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, TupleInputHole::new_mut(), self.8, self.9, self.10, self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut TupleInputHole, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator7> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P7: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut TupleInputHole, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.7).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, TupleInputHole::new_mut(), self.8, self.9, self.10, self.11))
	}
}

pub struct Disambiguator8 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut TupleInputHole, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator8> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P8: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut TupleInputHole, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.8).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, TupleInputHole::new_mut(), self.9, self.10, self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut TupleInputHole, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator8> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P8: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut TupleInputHole, &'a mut P9, &'a mut P10, &'a mut P11)) {
		((*self.8).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, TupleInputHole::new_mut(), self.9, self.10, self.11))
	}
}

pub struct Disambiguator9 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut TupleInputHole, &'a mut P10, &'a mut P11), Disambiguator9> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P9: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut TupleInputHole, &'a mut P10, &'a mut P11)) {
		((*self.9).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, TupleInputHole::new_mut(), self.10, self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut TupleInputHole, &'a mut P10, &'a mut P11), Disambiguator9> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P9: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut TupleInputHole, &'a mut P10, &'a mut P11)) {
		((*self.9).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, TupleInputHole::new_mut(), self.10, self.11))
	}
}

pub struct Disambiguator10 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut TupleInputHole, &'a mut P11), Disambiguator10> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P10: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut TupleInputHole, &'a mut P11)) {
		((*self.10).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, TupleInputHole::new_mut(), self.11))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut TupleInputHole, &'a mut P11), Disambiguator10> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P10: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut TupleInputHole, &'a mut P11)) {
		((*self.10).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, TupleInputHole::new_mut(), self.11))
	}
}

pub struct Disambiguator11 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut TupleInputHole), Disambiguator11> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P11: Borrow<T>,
{
	fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut TupleInputHole)) {
		((*self.11).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, TupleInputHole::new_mut()))
	}
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleDecompose<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut TupleInputHole), Disambiguator11> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
	P11: BorrowMut<T>,
{
	fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut TupleInputHole)) {
		((*self.11).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, TupleInputHole::new_mut()))
	}
}

// === Tuple Truncation === //

pub trait ArityTruncate<R> {
	fn truncate_arity(self) -> R;
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<()> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> () {
		()
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0,)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0,) {
		(self.0,)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1) {
		(self.0, self.1)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1, P2)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1, P2) {
		(self.0, self.1, self.2)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1, P2, P3)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1, P2, P3) {
		(self.0, self.1, self.2, self.3)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1, P2, P3, P4)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1, P2, P3, P4) {
		(self.0, self.1, self.2, self.3, self.4)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1, P2, P3, P4, P5)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1, P2, P3, P4, P5) {
		(self.0, self.1, self.2, self.3, self.4, self.5)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1, P2, P3, P4, P5, P6)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1, P2, P3, P4, P5, P6) {
		(self.0, self.1, self.2, self.3, self.4, self.5, self.6)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1, P2, P3, P4, P5, P6, P7)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1, P2, P3, P4, P5, P6, P7) {
		(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1, P2, P3, P4, P5, P6, P7, P8)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1, P2, P3, P4, P5, P6, P7, P8) {
		(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9) {
		(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10) {
		(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
	}
}

impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> ArityTruncate<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
	fn truncate_arity(self) -> (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
		(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
	}
}

// === Macro Definition === //

// FIXME: Unfortunately, `normalize_arity` limits the lifetime of the decomposed tuple to the lifetime
//  of the input tuple rather than the lifetimes of the tuple's elements.
// FIXME: Additionally, inference breaks down if the expression's type cannot be immediately resolved
//  (e.g. adding just one indirection between this expression and the assignment point at which the
//   output type is known is enough to break inference entirely)
#[macro_export]
macro_rules! decompose {
	($input:expr) => {
		{
			use $crate::macros::ArityExpand;
			let input = $input.normalize_arity();
			let builder = $crate::macros::TupleBuilder::new();
			
			match builder.inference_helper() {
				$crate::macros::Some(var) => var,
				$crate::macros::None => {
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p0, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p1, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p2, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p3, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p4, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p5, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p6, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p7, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p8, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p9, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p10, builder) = builder.id(v);
					
					let (v, input) = $crate::macros::TupleDecompose::search(input);
					let (p11, builder) = builder.id(v);
					

					let _builder = builder;
					let _input = input;

					$crate::macros::ArityTruncate::truncate_arity((p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11))
				}
			}
		}
	};
}
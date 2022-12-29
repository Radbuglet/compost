use core::{borrow::{Borrow, BorrowMut}, marker::PhantomData, ptr::NonNull};

// === Tuple Expansion === //

pub struct TupleRemainder<T>(pub T);

pub struct TupleInputHole {
    _private: (),
}

impl TupleInputHole {
    fn new_mut<'a>() -> &'a mut Self {
        unsafe {
            // Safety: ZSTs can dangle.
            NonNull::<Self>::dangling().as_mut()
        }
    }
}

// First, we define a mechanism for expanding all input tuples to tuples of the same arity.
pub trait NormalizeArity<'a> {
    type Target;

    fn normalize_arity(&'a mut self) -> Self::Target;
}

impl<'a> NormalizeArity<'a> for () {
    type Target = (&'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a> NormalizeArity<'a> for (P0,) {
    type Target = (&'a mut P0, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a> NormalizeArity<'a> for (P0, P1) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a, P2: 'a> NormalizeArity<'a> for (P0, P1, P2) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, &mut self.2, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a> NormalizeArity<'a> for (P0, P1, P2, P3) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a> NormalizeArity<'a> for (P0, P1, P2, P3, P4) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a> NormalizeArity<'a> for (P0, P1, P2, P3, P4, P5) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a> NormalizeArity<'a> for (P0, P1, P2, P3, P4, P5, P6) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a> NormalizeArity<'a> for (P0, P1, P2, P3, P4, P5, P6, P7) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a, P8: 'a> NormalizeArity<'a> for (P0, P1, P2, P3, P4, P5, P6, P7, P8) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut TupleInputHole, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, TupleInputHole::new_mut(), TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a, P8: 'a, P9: 'a> NormalizeArity<'a> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut TupleInputHole, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, TupleInputHole::new_mut(), TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a, P8: 'a, P9: 'a, P10: 'a> NormalizeArity<'a> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut TupleInputHole);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, TupleInputHole::new_mut())
    }
}

impl<'a, P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a, P8: 'a, P9: 'a, P10: 'a, P11: 'a> NormalizeArity<'a> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
    type Target = (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11);

    fn normalize_arity(&'a mut self) -> Self::Target {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11)
    }
}

impl<'a: 'b, 'b,P0: 'a, P1: 'a, P2: 'a, P3: 'a, P4: 'a, P5: 'a, P6: 'a, P7: 'a, P8: 'a, P9: 'a, P10: 'a, P11: 'a> NormalizeArity<'b> for TupleRemainder<(&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)> {
    type Target = (&'b mut P0, &'b mut P1, &'b mut P2, &'b mut P3, &'b mut P4, &'b mut P5, &'b mut P6, &'b mut P7, &'b mut P8, &'b mut P9, &'b mut P10, &'b mut P11);

    fn normalize_arity(&'b mut self) -> Self::Target {
        let me = &mut self.0;

        (&mut me.0, &mut me.1, &mut me.2, &mut me.3, &mut me.4, &mut me.5, &mut me.6, &mut me.7, &mut me.8, &mut me.9, &mut me.10, &mut me.11)
    }
}

// === Tuple output inference === //

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

pub trait TupleBuilderId<V, R>: Sized {
    fn id(self, v: V) -> (V, TupleBuilder<R>) {
        (v, TupleBuilder::new())
    }
}

impl TupleBuilderId<TupleOutputHole, ()> for TupleBuilder<()> {}

impl<P0> TupleBuilderId<P0, ()> for TupleBuilder<(P0,)> {}
impl<P0, P1> TupleBuilderId<P0, (P1,)> for TupleBuilder<(P0, P1)> {}
impl<P0, P1, P2> TupleBuilderId<P0, (P1, P2)> for TupleBuilder<(P0, P1, P2)> {}
impl<P0, P1, P2, P3> TupleBuilderId<P0, (P1, P2, P3)> for TupleBuilder<(P0, P1, P2, P3)> {}
impl<P0, P1, P2, P3, P4> TupleBuilderId<P0, (P1, P2, P3, P4)> for TupleBuilder<(P0, P1, P2, P3, P4)> {}
impl<P0, P1, P2, P3, P4, P5> TupleBuilderId<P0, (P1, P2, P3, P4, P5)> for TupleBuilder<(P0, P1, P2, P3, P4, P5)> {}
impl<P0, P1, P2, P3, P4, P5, P6> TupleBuilderId<P0, (P1, P2, P3, P4, P5, P6)> for TupleBuilder<(P0, P1, P2, P3, P4, P5, P6)> {}
impl<P0, P1, P2, P3, P4, P5, P6, P7> TupleBuilderId<P0, (P1, P2, P3, P4, P5, P6, P7)> for TupleBuilder<(P0, P1, P2, P3, P4, P5, P6, P7)> {}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> TupleBuilderId<P0, (P1, P2, P3, P4, P5, P6, P7, P8)> for TupleBuilder<(P0, P1, P2, P3, P4, P5, P6, P7, P8)> {}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> TupleBuilderId<P0, (P1, P2, P3, P4, P5, P6, P7, P8, P9)> for TupleBuilder<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)> {}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> TupleBuilderId<P0, (P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)> for TupleBuilder<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)> {}
impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleBuilderId<P0, (P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)> for TupleBuilder<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)> {}

// === Tuple searching === //

// Now, we define a way to search a tuple of arity `MAX_ARITY`.
pub trait TupleSearch<T, R, V> {
    fn search(self) -> (T, R);
}

impl<T> TupleSearch<TupleOutputHole, Self, ()> for T {
    fn search(self) -> (TupleOutputHole, Self) {
        (TupleOutputHole { _private: () }, self)
    }
}

pub struct Disambiguator0 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut TupleInputHole, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator0> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P0: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut TupleInputHole, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.0).borrow(), (TupleInputHole::new_mut(), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut TupleInputHole, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator0> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P0: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut TupleInputHole, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.0).borrow_mut(), (TupleInputHole::new_mut(), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

pub struct Disambiguator1 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut TupleInputHole, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator1> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P1: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut TupleInputHole, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.1).borrow(), (self.0, TupleInputHole::new_mut(), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut TupleInputHole, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator1> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P1: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut TupleInputHole, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.1).borrow_mut(), (self.0, TupleInputHole::new_mut(), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

pub struct Disambiguator2 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut P1, &'a mut TupleInputHole, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator2> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P2: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut TupleInputHole, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.2).borrow(), (self.0, self.1, TupleInputHole::new_mut(), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut TupleInputHole, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator2> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P2: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut TupleInputHole, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.2).borrow_mut(), (self.0, self.1, TupleInputHole::new_mut(), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

pub struct Disambiguator3 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut TupleInputHole, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator3> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P3: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut TupleInputHole, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.3).borrow(), (self.0, self.1, self.2, TupleInputHole::new_mut(), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut TupleInputHole, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator3> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P3: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut TupleInputHole, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.3).borrow_mut(), (self.0, self.1, self.2, TupleInputHole::new_mut(), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

pub struct Disambiguator4 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut TupleInputHole, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator4> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P4: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut TupleInputHole, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.4).borrow(), (self.0, self.1, self.2, self.3, TupleInputHole::new_mut(), self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut TupleInputHole, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator4> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P4: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut TupleInputHole, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.4).borrow_mut(), (self.0, self.1, self.2, self.3, TupleInputHole::new_mut(), self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

pub struct Disambiguator5 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut TupleInputHole, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator5> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P5: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut TupleInputHole, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.5).borrow(), (self.0, self.1, self.2, self.3, self.4, TupleInputHole::new_mut(), self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut TupleInputHole, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator5> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P5: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut TupleInputHole, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.5).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, TupleInputHole::new_mut(), self.6, self.7, self.8, self.9, self.10, self.11))
    }
}

pub struct Disambiguator6 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut TupleInputHole, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator6> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P6: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut TupleInputHole, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.6).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, TupleInputHole::new_mut(), self.7, self.8, self.9, self.10, self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut TupleInputHole, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator6> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P6: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut TupleInputHole, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.6).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, TupleInputHole::new_mut(), self.7, self.8, self.9, self.10, self.11))
    }
}

pub struct Disambiguator7 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut TupleInputHole, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator7> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P7: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut TupleInputHole, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.7).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, TupleInputHole::new_mut(), self.8, self.9, self.10, self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut TupleInputHole, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator7> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P7: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut TupleInputHole, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.7).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, TupleInputHole::new_mut(), self.8, self.9, self.10, self.11))
    }
}

pub struct Disambiguator8 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut TupleInputHole, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator8> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P8: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut TupleInputHole, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.8).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, TupleInputHole::new_mut(), self.9, self.10, self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut TupleInputHole, &'a mut P9, &'a mut P10, &'a mut P11), Disambiguator8> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P8: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut TupleInputHole, &'a mut P9, &'a mut P10, &'a mut P11)) {
        ((*self.8).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, TupleInputHole::new_mut(), self.9, self.10, self.11))
    }
}

pub struct Disambiguator9 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut TupleInputHole, &'a mut P10, &'a mut P11), Disambiguator9> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P9: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut TupleInputHole, &'a mut P10, &'a mut P11)) {
        ((*self.9).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, TupleInputHole::new_mut(), self.10, self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut TupleInputHole, &'a mut P10, &'a mut P11), Disambiguator9> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P9: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut TupleInputHole, &'a mut P10, &'a mut P11)) {
        ((*self.9).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, TupleInputHole::new_mut(), self.10, self.11))
    }
}

pub struct Disambiguator10 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut TupleInputHole, &'a mut P11), Disambiguator10> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P10: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut TupleInputHole, &'a mut P11)) {
        ((*self.10).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, TupleInputHole::new_mut(), self.11))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut TupleInputHole, &'a mut P11), Disambiguator10> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P10: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut TupleInputHole, &'a mut P11)) {
        ((*self.10).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, TupleInputHole::new_mut(), self.11))
    }
}

pub struct Disambiguator11 { _private: () }

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut TupleInputHole), Disambiguator11> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P11: Borrow<T>,
{
    fn search(self) -> (&'a  T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut TupleInputHole)) {
        ((*self.11).borrow(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, TupleInputHole::new_mut()))
    }
}

impl<'a, T: ?Sized, P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> TupleSearch<&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut TupleInputHole), Disambiguator11> for (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut P11)
where
    P11: BorrowMut<T>,
{
    fn search(self) -> (&'a mut T, (&'a mut P0, &'a mut P1, &'a mut P2, &'a mut P3, &'a mut P4, &'a mut P5, &'a mut P6, &'a mut P7, &'a mut P8, &'a mut P9, &'a mut P10, &'a mut TupleInputHole)) {
        ((*self.11).borrow_mut(), (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, TupleInputHole::new_mut()))
    }
}

// === Tuple truncation === //

// Finally, we define a mechanism for truncating the end off a tuple. This is used to get rid of the
// trailing `TupleOutputHole` instances emitted at the end of the tuple once all required components
// have been acquired.
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

// === Macro definition === //

/// This macro takes a **mutable reference** to a tuple and decomposes it into a sub-tuple
/// (i.e. a tuple containing a subset of the values contained in the parent tuple).
///
/// ## Syntax
///
/// There are three ways in which this macro can be used...
///
/// ##### ...in an **expression**:
///
/// [Jump to the "summary" section](#summary)
///
/// These forms are useful when calling a function with a desired subset of the available context.
///
/// ```
/// use compost::decompose;
///
/// let mut input = (1i32, &mut 2u32, 'c');
///
/// fn example(cx: (&i32, &mut u32)) {
///     assert_eq!(cx, (&1, &mut 2));
/// }
///
/// // Can be used when calling a function...
/// example(decompose!(input));
///
/// // ...or when assigning to a variable.
/// let cx_subset: (&mut u32, &mut char) = decompose!(input);
/// assert_eq!(cx_subset, (&mut 2, &mut 'c'));
///
/// // Which is equivalent to:
/// let cx_subset = decompose!(input => (&mut u32, &mut char));
/// assert_eq!(cx_subset, (&mut 2, &mut 'c'));
/// ```
///
/// ##### ...in a **statement:**
///
/// [Jump to the "summary" section](#summary)
///
/// These forms are useful for pulling context contained in a tuple into scope.
///
/// ```
/// use compost::decompose;
///
/// let mut input = (1i32, &mut 2u32, 'c', 5u8);
///
/// // Brings component references into scope and produces a `rest` value containing
/// //the remaining components.
/// //
/// // NOTE: Because `rest`'s tuple layout is unspecified, `rest` is new-typed in a
/// // macro-internal `TupleRemainder` struct to allow for backwards-compatibility-
/// // preserving changes to the maximum arity, the search mechanism, etc.
/// decompose!(input => rest & {
///     my_char: &mut char,
///     my_i32: &i32,
/// });
///
/// assert_eq!((my_char, my_i32), (&mut 'c', &1));
///
/// // `rest` can itself be decomposed several times.
/// decompose!(rest => rest & { my_u32: &u32 });
///
/// // If you're done decomposing, you can omit the `rest` parameter.
/// decompose!(rest => { my_u8: &mut u8 });
///
/// // (borrows from multiple decompose statements simultaneously)
/// assert_eq!((my_u32, my_u8), (&2, &mut 5));
/// assert_eq!(my_i32, &1);  // (remains valid!)
/// ```
///
/// ##### ...in an **expression** producing a "rest" tuple:
///
/// [Jump to the "summary" section](#summary)
///
/// These forms are useful for passing context to a method while allowing you to decompose the
/// remainder while the borrow is still ongoing.
///
///
/// ```
/// use compost::decompose;
///
/// #[derive(Debug)]
/// struct MyThing1<'a>(&'a mut i32, &'a mut u32);
///
/// impl<'a> MyThing1<'a> {
///     fn new((a, b): (&'a mut i32, &'a mut u32)) -> Self {
///         Self(a, b)
///     }
/// }
///
/// #[derive(Debug)]
/// struct MyThing2<'a>(&'a mut char);
///
/// impl<'a> MyThing2<'a> {
///     fn new((c,): (&'a mut char,)) -> Self {
///         Self(c)
///     }
/// }
///
/// fn do_something(mut cx: (&mut i32, &mut u32, &mut char, &str, &mut u8)) {
///     let (ctor_args, mut cx) = decompose!(...cx);
///     let thing_1 = MyThing1::new(ctor_args);
///
///     let (ctor_args, mut cx) = decompose!(...cx);
///     let thing_2 = MyThing2::new(ctor_args);
///
///     dbg!(&thing_1);
///     dbg!(&thing_2);
///
///     // This syntax can also be combined with the type-annotated tuple syntax.
///     let (the_str, mut cx) = decompose!(...cx => (&str));
///     dbg!(the_str);
///
///     let the_u8 = decompose!(cx => (&u8));
///     dbg!(the_u8);
/// }
///
/// do_something((&mut 1, &mut 2, &mut 'c', "d", &mut 5));
/// ```
///
/// ##### ...in a **combination of all three**:
///
/// [Jump to the "summary" section](#summary)
///
/// ```
/// use compost::decompose;
///
/// struct MyThing {
/// # /*
///     ...
/// # */
/// }
///
/// impl MyThing {
///     pub fn do_something<'a>(&mut self, deps: (&'a u32, &'a mut i32, &'a char)) -> &'a char {
///         dbg!(&deps);
///         deps.2
///     }
///
///     pub fn do_something_else(&mut self, deps: (&u8,)) {
///         dbg!(deps);
///     }
/// }
///
/// fn do_something(mut cx: (&mut MyThing, &mut u32, &mut i32, char, u8)) {
///     // Acquire a reference to the `MyThing` instance.
///     decompose!(cx => cx_rest & { thing: &mut MyThing });
///
///     // Call a method on it with even more context.
///     let (args, mut cx_rest) = decompose!(...cx_rest);
///     let my_char = thing.do_something(args);
///
///     // Call another unrelated method without rest decomposition.
///     thing.do_something_else(decompose!(cx_rest));
///
///     // `my_char` remains valid!
///     dbg!(my_char);
/// }
/// ```
///
/// ### Summary
///
/// In summary, here are the [expression forms](#in-an-expression) available for this macro:
///
/// - `decompose!($expr) -> (T1, T2, T3)`:<br>
///   Decomposes a tuple into a subset tuple.
///
/// - `decompose!($expr => ($T1, $T2, $T3)) -> (T1, T2, T3)`:<br>
///   Decomposes a tuple into a subset tuple with explicit type annotations.
///
/// Here are the [expression with rest forms](#in-an-expression-producing-a-rest-tuple) available for
/// this macro:
///
/// - `decompose!(...$expr) -> ((T1, T2, T3), TupleRemainder<_>)`:<br>
///   Decomposes a tuple into a subset tuple and the remainder of the input tuple after the borrow.
///
/// - `decompose!(...$expr => ($T1, $T2, $T3)) -> ((T1, T2, T3), TupleRemainder<_>)`:<br>
///   Decomposes a tuple into a subset tuple and the remainder of the input tuple after the borrow
///   with explicit type annotations.
///
/// And here are its [statement forms](#in-a-statement):
///
/// - `decompose!($expr => { $var_1: $T1, $var_2: $T2, $var_3: $T3 });`:<br>
///   Decomposes a tuple into `n` different components and brings them in scope under the specified
///   names.
///
/// - `decompose!($expr => $rest_name & { $var_1: $T1, $var_2: $T2, $var_3: $T3 });`:<br>
///   Decomposes a tuple into `n` different components and brings them in scope under the specified
///   names. Brings the remainder of the input tuple into scope under the specified `$rest_name`.
///
/// ## What Can Be Borrowed?
///
/// ##### Rule 1
///
/// `decompose!` expects a mutable reference to the tuple it is decomposing. Thus, this is not valid:
///
/// ```compile_fail
/// use compost::decompose;
///
/// fn example(cx: (&i32, &u32)) {
///     decompose!(cx => { my_i32: &i32 });
///     dbg!(my_i32);
/// }
/// ```
///
/// but this is:
///
/// ```
/// use compost::decompose;
///
/// fn example(mut cx: (&i32, &u32)) {  // (see how the `cx` variable itself is now mut?)
///     decompose!(cx => { my_i32: &i32 });
///     dbg!(my_i32);
/// }
/// ```
///
///  ##### Rule 2
///
/// `decompose!` always decomposes tuples into tuples, even if they're **single element tuples.**
///
/// Thus, this is not valid:
///
/// ```compile_fail
/// use compost::decompose;
///
/// fn takes_cx(cx: &i32) {
///     dbg!(cx);
/// }
///
/// fn example(mut cx: (&i32, &u32)) {
///     takes_cx(decompose!(cx));
/// }
/// ```
///
/// ...but this is:
///
/// ```
/// use compost::decompose;
///
/// fn takes_cx(cx: (&i32,)) {  // (notice the trailing comma?)
///     dbg!(cx);
/// }
///
/// fn example(mut cx: (&i32, &u32)) {
///     takes_cx(decompose!(cx));
/// }
/// ```
///
/// Note the **trailing comma** in `(&i32,)`, which differentiates single element tuples from
/// grouping parentheses around types.
///
///  ##### Rule 3
///
/// Components in the input tuple can be anything. They can be references, mutable references, smart pointers,
/// owned instances, etc. However, components in the output tuple must be **immutable or mutable** references.
///
/// A reference can be decomposed from an input tuple if the input tuple has some element that implements [`Borrow<T>`](Borrow)
/// (or [`BorrowMut<T>`](BorrowMut) if the reference being requested is mutable) to that specific type `T`.
///
/// ```
/// use core::borrow::Borrow;
/// use compost::decompose;
///
/// fn example<T: Borrow<V>, V>(mut cx: (T,)) {
///     decompose!(cx => { v: &V });
///
///     // Of course, you can still borrow the original value as well...
///     decompose!(cx => { v: &mut T });
/// }
/// ```
///
/// Note that the **actual element itself** must implement `Borrow` so, while `T: Borrow<V>`—
/// making it possible to decompose `&V` from an **owned** instance of `T`—`&'_ T` does not, making
/// that decomposition invalid. You'd need to adjust your generic parameter bounds to make that work:
///
/// ```
/// use core::borrow::Borrow;
/// use compost::decompose;
///
/// fn example<'a, T, V>(mut cx: (&'a mut T,))
/// where
///     &'a mut T: Borrow<V>,
///  {
///     decompose!(cx => { v: &V });
/// }
/// ```
///
///  ##### Rule 4
///
/// The element of the tuple providing the appropriate `Borrow` implementation must
/// be unambiguous. This implies, in the general case, that you cannot borrow an element when
/// that element is present multiple times in the input tuple:
///
/// ```compile_fail
/// use compost::decompose;
///
/// fn example(mut cx: (&i32, &mut i32, &u32, Box<u32>)) {
///     decompose!(cx => {
///         my_i32: &i32,
///         my_u32: &u32,
///     });
///     dbg!((my_i32, my_u32));
/// }
/// ```
///
/// Funnily enough, this works:
///
/// ```
/// use compost::decompose;
///
/// fn example(mut cx: (&i32, &mut i32, &u32, Box<u32>, &char, &char, &char)) {
///     decompose!(cx => {
///         // There's only one element in the input tuple that can give a **mutable
///         // reference** to these respective elements.
///         my_i32: &mut i32,
///         my_u32: &mut u32,
///
///         // Also, even though `&char` shows up *thrice* in the context tuple, it
///         // is not used anywhere in the decomposition so it is fine.
///     });
///     dbg!((my_i32, my_u32));
/// }
/// ```
///
///  ##### Rule 5
///
/// Finally, elements used in a tuple decomposition can only be used once,
/// even if **they could theoretically be shared.**
///
/// ```compile_fail
/// use compost::decompose;
///
/// fn example(mut cx: (&i32, &u32)) {
///     // This works well but...
///     decompose!(cx => rest & { my_first_i32_ref: &i32 });
///
///     // This fails!
///     decompose!(rest => { my_second_i32_ref: &i32 });
///
///     dbg!((my_first_i32_ref, my_second_i32_ref));
/// }
/// ```
///
/// ## Caveats
///
/// **Caveat 1:** Because variadic tuples are not a thing yet, the maximum arity of (number of elements in)
/// both the input and output tuples is **12**. This value is configurable in the source code
/// (see: `src/generated/decompose.rs.jinja`'s `MAX_ARITY` template variable).
///
/// **Caveat 2:** Because `decompose!` consumes a mutable reference to the tuple being decomposed:
///
/// 1. The tuple must be marked as mutable (but you already knew that).
/// 2. Tuple temporaries cannot be decomposed and returned from the function.
///
/// Thus, this fails to compile:
///
/// ```compile_fail
/// use compost::decompose;
///
/// fn give_me_some_things<'a>(mut cx: (&'a u32, &'a mut i32)) -> (&'a u32, &'a i32) {
///     decompose!(cx)
/// }
/// ```
#[macro_export]
macro_rules! decompose {
    // "Rest" decomposing expression
    (...$input:expr) => {
        {
            use $crate::macro_internal::NormalizeArity;
            let input = $input.normalize_arity();
            let builder = $crate::macro_internal::TupleBuilder::new();

            match builder.inference_helper() {
                $crate::macro_internal::Some(var) => {
                    fn any<T>() -> T {
                        loop {}
                    }
                    (var, any())
                },
                $crate::macro_internal::None => {
                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p0, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p1, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p2, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p3, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p4, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p5, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p6, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p7, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p8, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p9, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p10, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::TupleSearch::search(input);
                    let (p11, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let _builder = builder;

                    (
                        $crate::macro_internal::ArityTruncate::truncate_arity((p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)),
                        $crate::macro_internal::TupleRemainder(input)
                    )
                }
            }
        }
    };

    // Annotated "rest" decomposing expression
    (...$input:expr => (
        $($ty:ty),*$(,)?
    )) => {
        $crate::macro_internal::identity::<(
            ($($ty,)*),
            _,
        )>($crate::decompose!(...$input))
    };

    // Regular decomposing expression
    ($input:expr) => {
        $crate::decompose!(...$input).0
    };

    // Annotated regular decomposing expression
    ($input:expr => (
        $($ty:ty),*$(,)?
    )) => {
        $crate::macro_internal::identity::<($($ty,)*)>($crate::decompose!($input))
    };

    // "Rest" decomposing statement
    ($input:expr => $rest:ident & {
        $($name:ident: $ty:ty),*
        $(,)?
    }) => {
        #[allow(unnecessary_mut)]
        let (($($name,)*), mut $rest): (($($ty,)*), _) = $crate::decompose!(...$input);
    };

    // Regular decomposing statement
    ($input:expr => {
        $($name:ident: $ty:ty),*
        $(,)?
    }) => {
        $crate::decompose!($input => _ignored & { $($name:$ty),* });
    }
}

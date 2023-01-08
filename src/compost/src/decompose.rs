use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

// === ContextExtract === //

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct ContextHole;

pub trait ContextExtract<T, D> {
    type Rest;

    fn extract(self) -> (T, Self::Rest);
}

pub struct IdentityDisambiguator;

impl<'r, T: ?Sized> ContextExtract<&'r mut T, IdentityDisambiguator> for &'r mut T {
    type Rest = ContextHole;

    fn extract(self) -> (&'r mut T, Self::Rest) {
        (self, ContextHole)
    }
}

impl<'r, T: ?Sized> ContextExtract<&'r T, IdentityDisambiguator> for &'r mut T {
    type Rest = &'r T;

    fn extract(self) -> (&'r T, Self::Rest) {
        (self, self)
    }
}

impl<'r, T: ?Sized> ContextExtract<&'r T, IdentityDisambiguator> for &'r T {
    type Rest = &'r T;

    fn extract(self) -> (&'r T, Self::Rest) {
        (self, self)
    }
}

pub struct DerefDisambiguator;

impl<'r, T: ?Sized + DerefMut> ContextExtract<&'r mut T::Target, DerefDisambiguator> for &'r mut T {
    type Rest = ContextHole;

    fn extract(self) -> (&'r mut T::Target, Self::Rest) {
        (self, ContextHole)
    }
}

impl<'r, T: ?Sized + Deref> ContextExtract<&'r T::Target, DerefDisambiguator> for &'r mut T {
    type Rest = &'r T;

    fn extract(self) -> (&'r T::Target, Self::Rest) {
        (self, self)
    }
}

impl<'r, T: ?Sized + Deref> ContextExtract<&'r T::Target, DerefDisambiguator> for &'r T {
    type Rest = &'r T;

    fn extract(self) -> (&'r T::Target, Self::Rest) {
        (self, self)
    }
}

pub struct TupleDisambiguator0<D>(D);

impl<'r, T, D, P0, P1> ContextExtract<T, TupleDisambiguator0<D>> for (P0, P1)
where
    P0: ContextExtract<T, D>,
{
    type Rest = (P0::Rest, P1);

    fn extract(self) -> (T, Self::Rest) {
        let (v, r) = self.0.extract();
        (v, (r, self.1))
    }
}

pub struct TupleDisambiguator1<D>(D);

impl<'r, T, D, P0, P1> ContextExtract<T, TupleDisambiguator1<D>> for (P0, P1)
where
    P1: ContextExtract<T, D>,
{
    type Rest = (P0, P1::Rest);

    fn extract(self) -> (T, Self::Rest) {
        let (v, r) = self.1.extract();
        (v, (self.0, r))
    }
}

// === Reduce === //

pub trait Reduce {
    type Reduced;

    fn reduce(self) -> Self::Reduced;
}

impl<T: ReduceNonTerminal> Reduce for T {
    type Reduced = T::ReducedNonTerminal;

    fn reduce(self) -> Self::Reduced {
        self.reduce_non_terminal()
    }
}

impl Reduce for ContextHole {
    type Reduced = ();

    fn reduce(self) -> Self::Reduced {
        ()
    }
}

impl Reduce for () {
    type Reduced = ();

    fn reduce(self) -> Self::Reduced {
        ()
    }
}

pub trait ReduceNonTerminal {
    type ReducedNonTerminal;

    fn reduce_non_terminal(self) -> Self::ReducedNonTerminal;
}

impl<'r, T: ?Sized> ReduceNonTerminal for &'r T {
    type ReducedNonTerminal = Self;

    fn reduce_non_terminal(self) -> Self::ReducedNonTerminal {
        self
    }
}

impl<'r, T: ?Sized> ReduceNonTerminal for &'r mut T {
    type ReducedNonTerminal = Self;

    fn reduce_non_terminal(self) -> Self::ReducedNonTerminal {
        self
    }
}

impl<A: ReduceNonTerminal, B: ReduceNonTerminal> ReduceNonTerminal for (A, B) {
    type ReducedNonTerminal = (A::ReducedNonTerminal, B::ReducedNonTerminal);

    fn reduce_non_terminal(self) -> Self::ReducedNonTerminal {
        (self.0.reduce_non_terminal(), self.1.reduce_non_terminal())
    }
}

impl<A: ReduceNonTerminal> ReduceNonTerminal for (A, ContextHole) {
    type ReducedNonTerminal = A::ReducedNonTerminal;

    fn reduce_non_terminal(self) -> Self::ReducedNonTerminal {
        self.0.reduce_non_terminal()
    }
}

impl<B: ReduceNonTerminal> ReduceNonTerminal for (ContextHole, B) {
    type ReducedNonTerminal = B::ReducedNonTerminal;

    fn reduce_non_terminal(self) -> Self::ReducedNonTerminal {
        self.1.reduce_non_terminal()
    }
}

// === ConsTuple === //

pub trait ConsTuple<'r> {
    type Output;

    fn cons_tuple(&'r mut self) -> Self::Output;
}

impl<'p: 'r, 'r, T: ?Sized> ConsTuple<'r> for &'p T {
    type Output = &'r T;

    fn cons_tuple(&'r mut self) -> Self::Output {
        self
    }
}

impl<'p: 'r, 'r, T: ?Sized> ConsTuple<'r> for &'p mut T {
    type Output = &'r mut T;

    fn cons_tuple(&'r mut self) -> Self::Output {
        self
    }
}

impl<'r, P0: ConsTuple<'r>> ConsTuple<'r> for (P0,) {
    type Output = P0::Output;

    fn cons_tuple(&'r mut self) -> Self::Output {
        self.0.cons_tuple()
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>> ConsTuple<'r> for (P0, P1) {
    type Output = (P0::Output, P1::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        (self.0.cons_tuple(), self.1.cons_tuple())
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>, P2: ConsTuple<'r>> ConsTuple<'r> for (P0, P1, P2) {
    type Output = ((P0::Output, P1::Output), P2::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        ((self.0.cons_tuple(), self.1.cons_tuple()), self.2.cons_tuple())
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>, P2: ConsTuple<'r>, P3: ConsTuple<'r>> ConsTuple<'r> for (P0, P1, P2, P3) {
    type Output = (((P0::Output, P1::Output), P2::Output), P3::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        (((self.0.cons_tuple(), self.1.cons_tuple()), self.2.cons_tuple()), self.3.cons_tuple())
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>, P2: ConsTuple<'r>, P3: ConsTuple<'r>, P4: ConsTuple<'r>> ConsTuple<'r> for (P0, P1, P2, P3, P4) {
    type Output = ((((P0::Output, P1::Output), P2::Output), P3::Output), P4::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        ((((self.0.cons_tuple(), self.1.cons_tuple()), self.2.cons_tuple()), self.3.cons_tuple()), self.4.cons_tuple())
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>, P2: ConsTuple<'r>, P3: ConsTuple<'r>, P4: ConsTuple<'r>, P5: ConsTuple<'r>> ConsTuple<'r> for (P0, P1, P2, P3, P4, P5) {
    type Output = (((((P0::Output, P1::Output), P2::Output), P3::Output), P4::Output), P5::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        (((((self.0.cons_tuple(), self.1.cons_tuple()), self.2.cons_tuple()), self.3.cons_tuple()), self.4.cons_tuple()), self.5.cons_tuple())
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>, P2: ConsTuple<'r>, P3: ConsTuple<'r>, P4: ConsTuple<'r>, P5: ConsTuple<'r>, P6: ConsTuple<'r>> ConsTuple<'r> for (P0, P1, P2, P3, P4, P5, P6) {
    type Output = ((((((P0::Output, P1::Output), P2::Output), P3::Output), P4::Output), P5::Output), P6::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        ((((((self.0.cons_tuple(), self.1.cons_tuple()), self.2.cons_tuple()), self.3.cons_tuple()), self.4.cons_tuple()), self.5.cons_tuple()), self.6.cons_tuple())
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>, P2: ConsTuple<'r>, P3: ConsTuple<'r>, P4: ConsTuple<'r>, P5: ConsTuple<'r>, P6: ConsTuple<'r>, P7: ConsTuple<'r>> ConsTuple<'r> for (P0, P1, P2, P3, P4, P5, P6, P7) {
    type Output = (((((((P0::Output, P1::Output), P2::Output), P3::Output), P4::Output), P5::Output), P6::Output), P7::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        (((((((self.0.cons_tuple(), self.1.cons_tuple()), self.2.cons_tuple()), self.3.cons_tuple()), self.4.cons_tuple()), self.5.cons_tuple()), self.6.cons_tuple()), self.7.cons_tuple())
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>, P2: ConsTuple<'r>, P3: ConsTuple<'r>, P4: ConsTuple<'r>, P5: ConsTuple<'r>, P6: ConsTuple<'r>, P7: ConsTuple<'r>, P8: ConsTuple<'r>> ConsTuple<'r> for (P0, P1, P2, P3, P4, P5, P6, P7, P8) {
    type Output = ((((((((P0::Output, P1::Output), P2::Output), P3::Output), P4::Output), P5::Output), P6::Output), P7::Output), P8::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        ((((((((self.0.cons_tuple(), self.1.cons_tuple()), self.2.cons_tuple()), self.3.cons_tuple()), self.4.cons_tuple()), self.5.cons_tuple()), self.6.cons_tuple()), self.7.cons_tuple()), self.8.cons_tuple())
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>, P2: ConsTuple<'r>, P3: ConsTuple<'r>, P4: ConsTuple<'r>, P5: ConsTuple<'r>, P6: ConsTuple<'r>, P7: ConsTuple<'r>, P8: ConsTuple<'r>, P9: ConsTuple<'r>> ConsTuple<'r> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9) {
    type Output = (((((((((P0::Output, P1::Output), P2::Output), P3::Output), P4::Output), P5::Output), P6::Output), P7::Output), P8::Output), P9::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        (((((((((self.0.cons_tuple(), self.1.cons_tuple()), self.2.cons_tuple()), self.3.cons_tuple()), self.4.cons_tuple()), self.5.cons_tuple()), self.6.cons_tuple()), self.7.cons_tuple()), self.8.cons_tuple()), self.9.cons_tuple())
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>, P2: ConsTuple<'r>, P3: ConsTuple<'r>, P4: ConsTuple<'r>, P5: ConsTuple<'r>, P6: ConsTuple<'r>, P7: ConsTuple<'r>, P8: ConsTuple<'r>, P9: ConsTuple<'r>, P10: ConsTuple<'r>> ConsTuple<'r> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10) {
    type Output = ((((((((((P0::Output, P1::Output), P2::Output), P3::Output), P4::Output), P5::Output), P6::Output), P7::Output), P8::Output), P9::Output), P10::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        ((((((((((self.0.cons_tuple(), self.1.cons_tuple()), self.2.cons_tuple()), self.3.cons_tuple()), self.4.cons_tuple()), self.5.cons_tuple()), self.6.cons_tuple()), self.7.cons_tuple()), self.8.cons_tuple()), self.9.cons_tuple()), self.10.cons_tuple())
    }
}

impl<'r, P0: ConsTuple<'r>, P1: ConsTuple<'r>, P2: ConsTuple<'r>, P3: ConsTuple<'r>, P4: ConsTuple<'r>, P5: ConsTuple<'r>, P6: ConsTuple<'r>, P7: ConsTuple<'r>, P8: ConsTuple<'r>, P9: ConsTuple<'r>, P10: ConsTuple<'r>, P11: ConsTuple<'r>> ConsTuple<'r> for (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11) {
    type Output = (((((((((((P0::Output, P1::Output), P2::Output), P3::Output), P4::Output), P5::Output), P6::Output), P7::Output), P8::Output), P9::Output), P10::Output), P11::Output);

    fn cons_tuple(&'r mut self) -> Self::Output {
        (((((((((((self.0.cons_tuple(), self.1.cons_tuple()), self.2.cons_tuple()), self.3.cons_tuple()), self.4.cons_tuple()), self.5.cons_tuple()), self.6.cons_tuple()), self.7.cons_tuple()), self.8.cons_tuple()), self.9.cons_tuple()), self.10.cons_tuple()), self.11.cons_tuple())
    }
}

// === TupleBuilder === //

pub trait ContextExtractOrHole<T, D> {
    type OrHoleRest;

    fn extract_or_hole(self) -> (T, Self::OrHoleRest);
}

impl<E: ContextExtract<T, D>, T, D> ContextExtractOrHole<T, (D,)> for E {
    type OrHoleRest = E::Rest;

    fn extract_or_hole(self) -> (T, Self::OrHoleRest) {
        self.extract()
    }
}

impl<E> ContextExtractOrHole<TupleOutputHole, TupleOutputHole> for E {
    type OrHoleRest = E;

    fn extract_or_hole(self) -> (TupleOutputHole, Self) {
        (TupleOutputHole { _private: () }, self)
    }
}

pub struct TupleOutputHole {
    _private: (),
}

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

// === ArityTruncate === //

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

// === Decompose === //

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
/// let mut input = (&mut 1i32, &mut 2u32, &mut Box::new('c'));
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
/// let mut input = (&mut 1i32, &mut 2u32, (&mut 'c', &mut Box::new(5u8), &4f32));
///
/// // Brings component references into scope and produces a `rest` value containing the remaining
/// // components.
/// decompose!(input => rest & {
///     my_char: &mut char,
///     my_i32: &i32,
/// });
///
/// assert_eq!((my_char, my_i32), (&mut 'c', &1));
///
/// // `rest` can itself be decomposed several times.
/// // Note that we can borrow the `&i32` component immutably more than once.
/// decompose!(rest => rest & { my_i32_again: &i32, my_u32: &u32 });
///
/// assert_eq!(my_i32, my_i32_again);
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
/// ##### ...in a **combination of all of them**:
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
/// fn do_something(mut cx: (&mut MyThing, &mut u32, &mut i32, &mut char, &mut Box<u8>)) {
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
/// - `decompose!(...$expr) -> ((T1, T2, T3), <remainder binary tuple tree>)`:<br>
///   Decomposes a tuple into a subset tuple and the remainder of the input tuple after the borrow.
///
/// - `decompose!(...$expr => ($T1, $T2, $T3)) -> ((T1, T2, T3), <remainder binary tuple tree>)`:<br>
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
/// `decompose!` always consumes references (i.e. `&T` and `&mut T` but not smart pointers) and
/// (potentially nested) tuples containing references. It does not accept smart pointers directly
/// (e.g. `(Box<T>, Ref<T>)`)—you must give references to them instead (e.g. `(&mut Box<T>, &Ref<T>)`).
///
/// `decompose!` always produces single-level tuples (possibly with zero or one items).
///
/// Thus, this is not valid because we are decomposing into a single reference, not a tuple:
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
/// We can fix this by wrapping the component in a tuple with a single element:
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
/// Additionally, this is not valid because `Box<T>` is a smart-pointer, not a regular reference:
///
/// ```compile_fail
/// use compost::decompose;
///
/// fn takes_cx(cx: (&mut i32,)) {
///     dbg!(cx);
/// }
///
/// fn example(mut cx: (Box<i32>, &u32)) {
///     takes_cx(decompose!(cx));
/// }
/// ```
///
/// We can fix this by taking a mutable reference to the box instead:
///
/// ```
/// use compost::decompose;
///
/// fn takes_cx(cx: (&mut i32,)) {
///     dbg!(cx);
/// }
///
/// fn example(mut cx: (&mut Box<i32>, &u32)) {
///     takes_cx(decompose!(cx));
/// }
/// ```
///
///  ##### Rule 3
///
/// A reference can be decomposed from an input tuple if the input tuple has some element of that type
/// or implements [`Deref<T>`](Deref) to that specific type `T`.
///
/// ```
/// use core::ops::Deref;
/// use compost::decompose;
///
/// fn example<T: Deref<Target = V>, V>(mut cx: (&mut T,)) {
///     decompose!(cx => { v: &V });
///
///     // Of course, you can still borrow the original value as well...
///     decompose!(cx => { v: &mut T });
/// }
/// ```
///
/// Note that, currently, only one level of deref coercion is valid. Thus, this will not compile:
///
/// ```compile_fail
/// use compost::decompose;
///
/// fn consumer(cx: (&str,)) {
///     dbg!(cx);
/// }
///
/// let mut cx = (&mut Box::new("whee".to_string()), &mut 4u32);
///
/// consumer(decompose!(cx));
/// ```
///
/// but this will:
///
/// ```
/// use compost::decompose;
///
/// fn consumer(cx: (&str,)) {
///     dbg!(cx);
/// }
///
/// let mut cx = (&mut "whee".to_string(),);
///
/// consumer(decompose!(cx));
/// ```
///
///  ##### Rule 4
///
/// The element of the tuple being borrowed must be unambiguous. This implies, in the general case,
/// that you cannot borrow an element when that element is present multiple times in the input tuple:
///
/// ```compile_fail
/// use compost::decompose;
///
/// fn example(mut cx: (&i32, (&mut i32, &u32), &mut Box<u32>)) {
///     decompose!(cx => {
///         my_i32: &i32,  // Where do we get the `i32`?
///         my_u32: &u32,  // Where do we get the `u32`?
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
/// fn example(mut cx: (&i32, &mut i32, &u32, &mut Box<u32>, &char, (&char, &char))) {
///     decompose!(cx => {
///         // There's only one element in the input tuple that can give a **mutable reference** to
///         // these respective elements.
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
/// ## Caveats
///
/// **Caveat 1:** Because variadic tuples are not a thing yet, the maximum arity of (number of elements in)
/// both the input and output tuples is **12**. This value is configurable in the source code
/// (see: `src/decompose.rs.jinja`'s `MAX_ARITY` template variable). Note that you can still create
/// input tuples providing more than 12 elements by nesting them.
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
            use $crate::macro_internal::ConsTuple;
            let input = $input.cons_tuple();
            let builder = $crate::macro_internal::TupleBuilder::new();

            match builder.inference_helper() {
                $crate::macro_internal::Some(var) => {
                    fn any<T>() -> T {
                        loop {}
                    }
                    (var, any())
                },
                $crate::macro_internal::None => {
                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p0, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p1, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p2, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p3, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p4, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p5, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p6, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p7, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p8, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p9, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p10, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::macro_internal::ContextExtractOrHole::extract_or_hole(input);
                    let input = $crate::macro_internal::Reduce::reduce(input);
                    let (p11, builder) = $crate::macro_internal::TupleBuilderId::id(builder, v);

                    let _builder = builder;

                    (
                        $crate::macro_internal::ArityTruncate::truncate_arity((p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)),
                        input,
                    )
                }
            }
        }
    };

    // Annotated "rest" decomposing expression
    (...$input:expr => (
        $($ty:ty),*$(,)?
    )) => {
        {
            let result: (($($ty,)*), _) = $crate::decompose!(...$input);
            result
        }
    };

    // Regular decomposing expression
    ($input:expr) => {
        $crate::decompose!(...$input).0
    };

    // Annotated regular decomposing expression
    ($input:expr => (
        $($ty:ty),*$(,)?
    )) => {
        {
            let result: ($($ty,)*) = $crate::decompose!($input);
            result
        }
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
    };
}

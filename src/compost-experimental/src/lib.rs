#![no_std]
#![forbid(unsafe_code)]

mod decompose;

// === Context === //

pub trait Context<'r> {
    type Reborrowed;

    fn reborrow(&'r mut self) -> Self::Reborrowed;
}

impl<'r> Context<'r> for () {
    type Reborrowed = ();

    fn reborrow(&'r mut self) -> Self::Reborrowed {
        ()
    }
}

impl<'r> Context<'r> for ContextHole {
    type Reborrowed = ContextHole;

    fn reborrow(&'r mut self) -> Self::Reborrowed {
        ContextHole
    }
}

impl<'p: 'r, 'r, T: ?Sized> Context<'r> for &'p mut T {
    type Reborrowed = &'r mut T;

    fn reborrow(&'r mut self) -> Self::Reborrowed {
        self
    }
}

impl<'p: 'r, 'r, T: ?Sized> Context<'r> for &'p T {
    type Reborrowed = &'r T;

    fn reborrow(&'r mut self) -> Self::Reborrowed {
        self
    }
}

impl<'r, A: Context<'r>, B: Context<'r>> Context<'r> for (A, B) {
    type Reborrowed = (A::Reborrowed, B::Reborrowed);

    fn reborrow(&'r mut self) -> Self::Reborrowed {
        (self.0.reborrow(), self.1.reborrow())
    }
}

// === ContextExtract === //

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct ContextHole;

pub trait ContextExtract<'r, T, D>: Context<'r> {
    type Rest;

    fn extract(&'r mut self) -> (T, Self::Rest);
}

#[doc(hidden)]
pub mod simple_context_impls {
    use super::*;

    use core::ops;

    pub struct Identity;

    impl<'r, T> ContextExtract<'r, &'r mut T, Identity> for &'r mut T {
        type Rest = ContextHole;

        fn extract(&'r mut self) -> (&'r mut T, Self::Rest) {
            (self, ContextHole)
        }
    }

    impl<'r, T> ContextExtract<'r, &'r T, Identity> for &'r mut T {
        type Rest = &'r T;

        fn extract(&'r mut self) -> (&'r T, Self::Rest) {
            (self, self)
        }
    }

    impl<'r, T> ContextExtract<'r, &'r T, Identity> for &'r T {
        type Rest = &'r T;

        fn extract(&'r mut self) -> (&'r T, Self::Rest) {
            (self, self)
        }
    }

    pub struct Deref;

    impl<'r, T: ops::DerefMut> ContextExtract<'r, &'r mut T::Target, Deref> for &'r mut T {
        type Rest = ContextHole;

        fn extract(&'r mut self) -> (&'r mut T::Target, Self::Rest) {
            (self, ContextHole)
        }
    }

    impl<'r, T: ops::Deref> ContextExtract<'r, &'r T::Target, Deref> for &'r mut T {
        type Rest = &'r T;

        fn extract(&'r mut self) -> (&'r T::Target, Self::Rest) {
            (self, self)
        }
    }

    impl<'r, T: ops::Deref> ContextExtract<'r, &'r T::Target, Deref> for &'r T {
        type Rest = &'r T;

        fn extract(&'r mut self) -> (&'r T::Target, Self::Rest) {
            (self, self)
        }
    }
}

#[doc(hidden)]
pub mod composite_context_impls {
    use super::*;

    pub struct TupleDisambiguator0<D>(D);

    impl<'r, T, D, P0, P1> ContextExtract<'r, T, TupleDisambiguator0<D>> for (P0, P1)
    where
        P0: Context<'r> + ContextExtract<'r, T, D>,
        P1: Context<'r>,
    {
        type Rest = (P0::Rest, P1::Reborrowed);

        fn extract(&'r mut self) -> (T, Self::Rest) {
            let (v, r) = self.0.extract();
            (v, (r, self.1.reborrow()))
        }
    }

    pub struct TupleDisambiguator1<D>(D);

    impl<'r, T, D, P0, P1> ContextExtract<'r, T, TupleDisambiguator1<D>> for (P0, P1)
    where
        P0: Context<'r>,
        P1: ContextExtract<'r, T, D>,
    {
        type Rest = (P0::Reborrowed, P1::Rest);

        fn extract(&'r mut self) -> (T, Self::Rest) {
            let (v, r) = self.1.extract();
            (v, (self.0.reborrow(), r))
        }
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

// === Playground === //

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

// === `decompose` macro === //

#[doc(hidden)]
pub mod decompose_macro_internal {
    pub use super::{decompose::*, Reduce};
    pub use core::{
        convert::identity,
        option::Option::{None, Some},
    };
}

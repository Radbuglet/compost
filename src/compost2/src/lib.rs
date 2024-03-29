#![no_std]
#![forbid(unsafe_code)]

// === Context === //

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct NoValue;
use NoValue as N;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Cx<A = N, B = N, C = N, D = N, E = N, F = N, G = N, H = N, I = N, J = N, K = N, L = N>(
    pub A,
    pub B,
    pub C,
    pub D,
    pub E,
    pub F,
    pub G,
    pub H,
    pub I,
    pub J,
    pub K,
    pub L,
);

impl From<()> for Cx {
    fn from((): ()) -> Self {
        Self(N, N, N, N, N, N, N, N, N, N, N, N)
    }
}

impl<A> From<(A,)> for Cx<A> {
    fn from((a,): (A,)) -> Self {
        Self(a, N, N, N, N, N, N, N, N, N, N, N)
    }
}

impl<A, B> From<(A, B)> for Cx<A, B> {
    fn from((a, b): (A, B)) -> Self {
        Self(a, b, N, N, N, N, N, N, N, N, N, N)
    }
}

impl<A, B, C> From<(A, B, C)> for Cx<A, B, C> {
    fn from((a, b, c): (A, B, C)) -> Self {
        Self(a, b, c, N, N, N, N, N, N, N, N, N)
    }
}

impl<A, B, C, D> From<(A, B, C, D)> for Cx<A, B, C, D> {
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        Self(a, b, c, d, N, N, N, N, N, N, N, N)
    }
}

impl<A, B, C, D, E> From<(A, B, C, D, E)> for Cx<A, B, C, D, E> {
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> Self {
        Self(a, b, c, d, e, N, N, N, N, N, N, N)
    }
}

impl<A, B, C, D, E, F> From<(A, B, C, D, E, F)> for Cx<A, B, C, D, E, F> {
    fn from((a, b, c, d, e, f): (A, B, C, D, E, F)) -> Self {
        Self(a, b, c, d, e, f, N, N, N, N, N, N)
    }
}

impl<A, B, C, D, E, F, G> From<(A, B, C, D, E, F, G)> for Cx<A, B, C, D, E, F, G> {
    fn from((a, b, c, d, e, f, g): (A, B, C, D, E, F, G)) -> Self {
        Self(a, b, c, d, e, f, g, N, N, N, N, N)
    }
}

impl<A, B, C, D, E, F, G, H> From<(A, B, C, D, E, F, G, H)> for Cx<A, B, C, D, E, F, G, H> {
    fn from((a, b, c, d, e, f, g, h): (A, B, C, D, E, F, G, H)) -> Self {
        Self(a, b, c, d, e, f, g, h, N, N, N, N)
    }
}

impl<A, B, C, D, E, F, G, H, I> From<(A, B, C, D, E, F, G, H, I)>
    for Cx<A, B, C, D, E, F, G, H, I>
{
    fn from((a, b, c, d, e, f, g, h, i): (A, B, C, D, E, F, G, H, I)) -> Self {
        Self(a, b, c, d, e, f, g, h, i, N, N, N)
    }
}

impl<A, B, C, D, E, F, G, H, I, J> From<(A, B, C, D, E, F, G, H, I, J)>
    for Cx<A, B, C, D, E, F, G, H, I, J>
{
    fn from((a, b, c, d, e, f, g, h, i, j): (A, B, C, D, E, F, G, H, I, J)) -> Self {
        Self(a, b, c, d, e, f, g, h, i, j, N, N)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> From<(A, B, C, D, E, F, G, H, I, J, K)>
    for Cx<A, B, C, D, E, F, G, H, I, J, K>
{
    fn from((a, b, c, d, e, f, g, h, i, j, k): (A, B, C, D, E, F, G, H, I, J, K)) -> Self {
        Self(a, b, c, d, e, f, g, h, i, j, k, N)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> From<(A, B, C, D, E, F, G, H, I, J, K, L)>
    for Cx<A, B, C, D, E, F, G, H, I, J, K, L>
{
    fn from((a, b, c, d, e, f, g, h, i, j, k, l): (A, B, C, D, E, F, G, H, I, J, K, L)) -> Self {
        Self(a, b, c, d, e, f, g, h, i, j, k, l)
    }
}

// === Macro Helpers === //

pub mod macro_internals {
    use core::ops::Deref;

    use crate::{Cx, NoValue};

    // Unreachable
    pub fn falsely_produce_any_cx_for_placeholder<A, B, C, D, E, F, G, H, I, J, K, L>(
    ) -> Cx<A, B, C, D, E, F, G, H, I, J, K, L> {
        unreachable!(
			"Compost built with the `compost_use_cx_placeholder` cfg, which replaces `cx!`'s expanded \
			 output with a non-functional placeholder to reduce check times. To actually be able to \
			 run the application, please rebuild it without the `compost_use_cx_placeholder` cfg flag."
		);
    }

    pub fn falsely_produce_anything<T>() -> T {
        unreachable!();
    }

    // assert_is_context
    pub fn assert_is_context<A, B, C, D, E, F, G, H, I, J, K, L>(
        _cx: &Cx<A, B, C, D, E, F, G, H, I, J, K, L>,
    ) -> ! {
        unreachable!();
    }

    pub fn assert_is_context_and_bind<A, B, C, D, E, F, G, H, I, J, K, L>(
        _cx: &Cx<A, B, C, D, E, F, G, H, I, J, K, L>,
        _binds: &(
            Binder<A>,
            Binder<B>,
            Binder<C>,
            Binder<D>,
            Binder<E>,
            Binder<F>,
            Binder<G>,
            Binder<H>,
            Binder<I>,
            Binder<J>,
            Binder<K>,
            Binder<L>,
        ),
    ) -> ! {
        unreachable!();
    }

    // Binder
    pub struct Binder<T>([T; 0]);

    impl<T> Binder<T> {
        pub fn new() -> Self {
            Self([])
        }

        pub fn fake_make(&self) -> T {
            unreachable!();
        }
    }

    // ContextAsksFor
    pub struct ContextAsksFor1<C, V>(ContextAsksFor2<C, V>);
    pub struct ContextAsksFor2<C, V>(ContextAsksFor3<C, V>);
    pub struct ContextAsksFor3<C, V>([(C, V); 0]);

    impl<C, V> ContextAsksFor1<C, V> {
        pub fn new(_c: &Binder<C>, _v: &Binder<V>) -> Self {
            Self(ContextAsksFor2(ContextAsksFor3([])))
        }
    }

    impl<C, V> Deref for ContextAsksFor1<C, V> {
        type Target = ContextAsksFor2<C, V>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<C, V> Deref for ContextAsksFor2<C, V> {
        type Target = ContextAsksFor3<C, V>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    pub trait ContextAsksForResolverRejected {
        fn is_asked_for(&self) -> bool {
            false
        }

        fn panic_if_not_injecting_it_in(&self) -> ! {
            unreachable!();
        }

        fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2, T>(
            &self,
            _curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
            _value: T,
        ) -> ! {
            unreachable!();
        }

        fn do_not_inject_it_in<T>(&self, t: T) -> T {
            t
        }
    }

    pub trait ContextAsksForResolverSuccess<Disamb> {
        type Borrowed: ?Sized;

        type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

        fn is_asked_for(&self) -> bool {
            true
        }

        fn panic_if_not_injecting_it_in(&self) {}

        fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
            &self,
            curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
            value: Self::Borrowed,
        ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

        fn do_not_inject_it_in<T>(&self, _t: T) -> ! {
            unreachable!()
        }
    }

    impl<C> ContextAsksForResolverRejected for ContextAsksFor1<C, NoValue> {}

    impl<C, V> ContextAsksForResolverRejected for ContextAsksFor3<C, V> {}

    pub struct DisambiguatorA;
    pub struct DisambiguatorB;
    pub struct DisambiguatorC;
    pub struct DisambiguatorD;
    pub struct DisambiguatorE;
    pub struct DisambiguatorF;
    pub struct DisambiguatorG;
    pub struct DisambiguatorH;
    pub struct DisambiguatorI;
    pub struct DisambiguatorJ;
    pub struct DisambiguatorK;
    pub struct DisambiguatorL;

    mod mut2mut {
        use super::*;

        impl<'a, A: ?Sized, B, C, D, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorA>
            for ContextAsksFor2<Cx<&'a mut A, B, C, D, E, F, G, H, I, J, K, L>, &'a mut A>
        {
            type Borrowed = &'a mut A;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<&'a mut A, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    value,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B: ?Sized, C, D, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorB>
            for ContextAsksFor2<Cx<A, &'a mut B, C, D, E, F, G, H, I, J, K, L>, &'a mut B>
        {
            type Borrowed = &'a mut B;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, &'a mut B, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    value,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C: ?Sized, D, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorC>
            for ContextAsksFor2<Cx<A, B, &'a mut C, D, E, F, G, H, I, J, K, L>, &'a mut C>
        {
            type Borrowed = &'a mut C;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, &'a mut C, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    value,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D: ?Sized, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorD>
            for ContextAsksFor2<Cx<A, B, C, &'a mut D, E, F, G, H, I, J, K, L>, &'a mut D>
        {
            type Borrowed = &'a mut D;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, &'a mut D, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    value,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E: ?Sized, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorE>
            for ContextAsksFor2<Cx<A, B, C, D, &'a mut E, F, G, H, I, J, K, L>, &'a mut E>
        {
            type Borrowed = &'a mut E;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, &'a mut E, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    value,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F: ?Sized, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorF>
            for ContextAsksFor2<Cx<A, B, C, D, E, &'a mut F, G, H, I, J, K, L>, &'a mut F>
        {
            type Borrowed = &'a mut F;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, &'a mut F, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    value,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G: ?Sized, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorG>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, &'a mut G, H, I, J, K, L>, &'a mut G>
        {
            type Borrowed = &'a mut G;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, &'a mut G, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    value,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H: ?Sized, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorH>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, &'a mut H, I, J, K, L>, &'a mut H>
        {
            type Borrowed = &'a mut H;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, &'a mut H, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    value,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I: ?Sized, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorI>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, &'a mut I, J, K, L>, &'a mut I>
        {
            type Borrowed = &'a mut I;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, &'a mut I, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    value,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I, J: ?Sized, K, L>
            ContextAsksForResolverSuccess<DisambiguatorJ>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, I, &'a mut J, K, L>, &'a mut J>
        {
            type Borrowed = &'a mut J;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, &'a mut J, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    value,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I, J, K: ?Sized, L>
            ContextAsksForResolverSuccess<DisambiguatorK>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, I, J, &'a mut K, L>, &'a mut K>
        {
            type Borrowed = &'a mut K;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, &'a mut K, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    value,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I, J, K, L: ?Sized>
            ContextAsksForResolverSuccess<DisambiguatorL>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, I, J, K, &'a mut L>, &'a mut L>
        {
            type Borrowed = &'a mut L;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, &'a mut L>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    value,
                )
            }
        }
    }

    mod mut2ref {
        use super::*;

        impl<'a, A: ?Sized, B, C, D, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorA>
            for ContextAsksFor2<Cx<&'a A, B, C, D, E, F, G, H, I, J, K, L>, &'a mut A>
        {
            type Borrowed = &'a A;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<&'a A, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    value,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B: ?Sized, C, D, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorB>
            for ContextAsksFor2<Cx<A, &'a B, C, D, E, F, G, H, I, J, K, L>, &'a mut B>
        {
            type Borrowed = &'a B;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, &'a B, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    value,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C: ?Sized, D, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorC>
            for ContextAsksFor2<Cx<A, B, &'a C, D, E, F, G, H, I, J, K, L>, &'a mut C>
        {
            type Borrowed = &'a C;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, &'a C, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    value,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D: ?Sized, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorD>
            for ContextAsksFor2<Cx<A, B, C, &'a D, E, F, G, H, I, J, K, L>, &'a mut D>
        {
            type Borrowed = &'a D;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, &'a D, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    value,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E: ?Sized, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorE>
            for ContextAsksFor2<Cx<A, B, C, D, &'a E, F, G, H, I, J, K, L>, &'a mut E>
        {
            type Borrowed = &'a E;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, &'a E, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    value,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F: ?Sized, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorF>
            for ContextAsksFor2<Cx<A, B, C, D, E, &'a F, G, H, I, J, K, L>, &'a mut F>
        {
            type Borrowed = &'a F;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, &'a F, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    value,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G: ?Sized, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorG>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, &'a G, H, I, J, K, L>, &'a mut G>
        {
            type Borrowed = &'a G;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, &'a G, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    value,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H: ?Sized, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorH>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, &'a H, I, J, K, L>, &'a mut H>
        {
            type Borrowed = &'a H;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, &'a H, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    value,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I: ?Sized, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorI>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, &'a I, J, K, L>, &'a mut I>
        {
            type Borrowed = &'a I;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, &'a I, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    value,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I, J: ?Sized, K, L>
            ContextAsksForResolverSuccess<DisambiguatorJ>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, I, &'a J, K, L>, &'a mut J>
        {
            type Borrowed = &'a J;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, &'a J, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    value,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I, J, K: ?Sized, L>
            ContextAsksForResolverSuccess<DisambiguatorK>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, I, J, &'a K, L>, &'a mut K>
        {
            type Borrowed = &'a K;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, &'a K, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    value,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I, J, K, L: ?Sized>
            ContextAsksForResolverSuccess<DisambiguatorL>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, I, J, K, &'a L>, &'a mut L>
        {
            type Borrowed = &'a L;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, &'a L>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    value,
                )
            }
        }
    }

    mod ref2ref {
        use super::*;

        impl<'a, A: ?Sized, B, C, D, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorA>
            for ContextAsksFor2<Cx<&'a A, B, C, D, E, F, G, H, I, J, K, L>, &'a A>
        {
            type Borrowed = &'a A;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<&'a A, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    value,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B: ?Sized, C, D, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorB>
            for ContextAsksFor2<Cx<A, &'a B, C, D, E, F, G, H, I, J, K, L>, &'a B>
        {
            type Borrowed = &'a B;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, &'a B, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    value,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C: ?Sized, D, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorC>
            for ContextAsksFor2<Cx<A, B, &'a C, D, E, F, G, H, I, J, K, L>, &'a C>
        {
            type Borrowed = &'a C;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, &'a C, D2, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    value,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D: ?Sized, E, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorD>
            for ContextAsksFor2<Cx<A, B, C, &'a D, E, F, G, H, I, J, K, L>, &'a D>
        {
            type Borrowed = &'a D;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, &'a D, E2, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    value,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E: ?Sized, F, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorE>
            for ContextAsksFor2<Cx<A, B, C, D, &'a E, F, G, H, I, J, K, L>, &'a E>
        {
            type Borrowed = &'a E;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, &'a E, F2, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    value,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F: ?Sized, G, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorF>
            for ContextAsksFor2<Cx<A, B, C, D, E, &'a F, G, H, I, J, K, L>, &'a F>
        {
            type Borrowed = &'a F;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, &'a F, G2, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    value,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G: ?Sized, H, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorG>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, &'a G, H, I, J, K, L>, &'a G>
        {
            type Borrowed = &'a G;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, &'a G, H2, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    value,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H: ?Sized, I, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorH>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, &'a H, I, J, K, L>, &'a H>
        {
            type Borrowed = &'a H;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, &'a H, I2, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    value,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I: ?Sized, J, K, L>
            ContextAsksForResolverSuccess<DisambiguatorI>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, &'a I, J, K, L>, &'a I>
        {
            type Borrowed = &'a I;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, &'a I, J2, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    value,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I, J: ?Sized, K, L>
            ContextAsksForResolverSuccess<DisambiguatorJ>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, I, &'a J, K, L>, &'a J>
        {
            type Borrowed = &'a J;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, &'a J, K2, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    value,
                    curr_borrow_cx.10,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I, J, K: ?Sized, L>
            ContextAsksForResolverSuccess<DisambiguatorK>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, I, J, &'a K, L>, &'a K>
        {
            type Borrowed = &'a K;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, &'a K, L2>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    value,
                    curr_borrow_cx.11,
                )
            }
        }

        impl<'a, A, B, C, D, E, F, G, H, I, J, K, L: ?Sized>
            ContextAsksForResolverSuccess<DisambiguatorL>
            for ContextAsksFor2<Cx<A, B, C, D, E, F, G, H, I, J, K, &'a L>, &'a L>
        {
            type Borrowed = &'a L;
            type InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> =
                Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, &'a L>;

            fn inject_it_in<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>(
                &self,
                curr_borrow_cx: Cx<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2>,
                value: Self::Borrowed,
            ) -> Self::InjectedIn<A2, B2, C2, D2, E2, F2, G2, H2, I2, J2, K2, L2> {
                Cx(
                    curr_borrow_cx.0,
                    curr_borrow_cx.1,
                    curr_borrow_cx.2,
                    curr_borrow_cx.3,
                    curr_borrow_cx.4,
                    curr_borrow_cx.5,
                    curr_borrow_cx.6,
                    curr_borrow_cx.7,
                    curr_borrow_cx.8,
                    curr_borrow_cx.9,
                    curr_borrow_cx.10,
                    value,
                )
            }
        }
    }
}

#[cfg(not(all(rust_analyzer, not(check_compost_borrows_in_rust_analyzer))))]
#[macro_export]
macro_rules! cx {
    (dangerously_specify_place $({$($cx:tt)*}),+$(,)?) => {
        'a: {
            #[allow(unused_imports)]
            use $crate::macro_internals::{
                ContextAsksForResolverRejected as _, ContextAsksForResolverSuccess as _,
            };

            // Determine the type of the expected context
            let res_bind = $crate::macro_internals::Binder::new();
            if false {
                break 'a res_bind.fake_make();
            }

            if false {
                $crate::macro_internals::assert_is_context(&res_bind.fake_make());
            }

			// Construct the resulting context
			let new_context = $crate::Cx::from(());

			$(
				// Ensure that the `$cx` identifier points to an appropriate context.
				let cx_binds = (
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
					$crate::macro_internals::Binder::new(),
				);

				#[allow(unreachable_code)]
				if false {
					// Returns `!` so the borrow doesn't disrupt anything.
					loop {}
					$crate::macro_internals::assert_is_context_and_bind(&$($cx)*, &cx_binds);
				}

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.0);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.0)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.1);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.1)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.2);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.2)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.3);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.3)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.4);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.4)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.5);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.5)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.6);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.6)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.7);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.7)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.8);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.8)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.9);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.9)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.10);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.10)
				} else {
					ask.do_not_inject_it_in(new_context)
				};

				let ask = $crate::macro_internals::ContextAsksFor1::new(&res_bind, &cx_binds.11);
				let new_context = if ask.is_asked_for() {
					ask.panic_if_not_injecting_it_in();
					#[allow(unreachable_code)]
					ask.inject_it_in(new_context, $($cx)*.11)
				} else {
					ask.do_not_inject_it_in(new_context)
				};
			)*

            new_context
        }
    };
	($($cx:ident),*$(,)?) => {
		$crate::cx!(dangerously_specify_place $({$cx}),*)
	};
}

#[cfg(all(rust_analyzer, not(check_compost_borrows_in_rust_analyzer)))]
#[macro_export]
macro_rules! cx {
    (dangerously_specify_place $({$($cx:tt)*}),+$(,)?) => {
        {
			// Ensure that the provided input is actually a `Cx` object to perform some level of
			// useful typechecking.
            #[allow(unreachable_code)]
			if false {
				// Returns `!` so the borrow doesn't disrupt anything.
				loop {}
				$($crate::macro_internals::assert_is_context_and_bind(
					&$($cx)*,
					&$crate::macro_internals::falsely_produce_anything(),
				);)*
			}

			// Ensure that the output is some `Cx` object to perform some level of useful typechecking.
			$crate::macro_internals::falsely_produce_any_cx_for_placeholder()
        }
    };
	($($cx:ident),*$(,)?) => {
		$crate::cx!(dangerously_specify_place $({$cx}),*)
	};
}

// === Tests === //

#[cfg(test)]
#[allow(dead_code)]
fn tests() {
    fn produce<T>() -> T {
        unreachable!()
    }

    fn extract<T: ?Sized>(t: Cx<&T>) -> &T {
        t.0
    }

    fn extract_mut<T: ?Sized>(t: Cx<&mut T>) -> &mut T {
        t.0
    }

    let cx: Cx<&mut u32, &i32, &mut str, &mut f32> = produce();
    let a = extract::<u32>(cx!(cx));
    let b = extract::<u32>(cx!(cx));
    let _ = (a, b);

    let c = extract_mut::<u32>(cx!(cx));
    let d = extract::<i32>(cx!(cx));
    let _ = (c, d);

    let e = extract_mut::<str>(cx!(cx));
    let _ = (d, e);

    let cx_2: (Cx<&mut u32>,) = produce();
    let a = extract::<u32>(cx!(dangerously_specify_place { cx_2.0 }));
    let b = extract::<u32>(cx!(dangerously_specify_place { cx_2.0 }));
    let _ = (a, b);

    struct Lt<'a>([&'a (); 0]);

    impl Drop for Lt<'_> {
        fn drop(&mut self) {}
    }

    fn borrow<'a, T: 'a>(_cx: Cx<&'a mut T>) -> Lt<'a> {
        unreachable!();
    }

    let a = borrow::<u32>(cx!(cx));
    let b = borrow::<f32>(cx!(cx));
    drop((a, b));
}

use core::marker::PhantomData;
use crate::{Context, ContextExtract};

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

pub trait ContextExtractOrHole<'r, T, D> {
	type OrHoleRest;

	fn extract_or_hole(&'r mut self) -> (T, Self::OrHoleRest);
}

impl<'r, E: ContextExtract<'r, T, D>, T, D> ContextExtractOrHole<'r, T, (D,)> for E {
	type OrHoleRest = E::Rest;

	fn extract_or_hole(&'r mut self) -> (T, Self::OrHoleRest) {
		self.extract()
	}
}

impl<'r, E: Context<'r>> ContextExtractOrHole<'r, TupleOutputHole, TupleOutputHole> for E {
	type OrHoleRest = E::Reborrowed;

	fn extract_or_hole(&'r mut self) -> (TupleOutputHole, Self::OrHoleRest) {
		(TupleOutputHole { _private: () }, self.reborrow())
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

#[macro_export]
macro_rules! decompose {
    // "Rest" decomposing expression
    (...$input:expr) => {
        {
            use $crate::decompose_macro_internal::ConsTuple;
            let mut input = $input.cons_tuple();
            let builder = $crate::decompose_macro_internal::TupleBuilder::new();

            match builder.inference_helper() {
                $crate::decompose_macro_internal::Some(var) => {
                    fn any<T>() -> T {
                        loop {}
                    }
                    (var, any())
                },
                $crate::decompose_macro_internal::None => {
                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p0, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p1, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p2, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p3, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p4, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p5, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p6, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p7, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p8, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p9, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p10, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let (v, input) = $crate::decompose_macro_internal::ContextExtractOrHole::extract_or_hole(&mut input);
					let mut input = $crate::decompose_macro_internal::Reduce::reduce(input);
                    let (p11, builder) = $crate::decompose_macro_internal::TupleBuilderId::id(builder, v);

                    let _builder = builder;

                    (
                        $crate::decompose_macro_internal::ArityTruncate::truncate_arity((p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)),
                        input,
                    )
                }
            }
        }
    };
}
use compost::decompose;

fn main() {
	let mut cx = (&1u32, &mut 2i32, 'c', "d", Box::new(5u8), 6i8);

	fn function_taking_subset(cx: (&u32, &i32, &mut u8)) {
		dbg!(cx);
	}

	function_taking_subset(decompose!(cx));

	decompose!(cx => cx_rest & {
		value_1: &str,
		value_2: &mut char,
	});

	dbg!((value_1, value_2));

	decompose!(cx_rest => { value_3: &u32 });
	dbg!(value_3);

	function_taking_subset(decompose!(cx_rest));
	function_taking_subset(decompose!(cx));
}

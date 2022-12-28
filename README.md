# Compost

This library exposes `decompose!`, a macro to decompose tuples into tuples containing a
subset of their values.

```rust
use compost::decompose;

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

decompose!(cx_rest => { value_3: &u32, value_4: &mut i8 });

dbg!((value_3, value_4));

function_taking_subset(decompose!(cx_rest));
function_taking_subset(decompose!(cx));
```

## Features

Yes, this library...

- Supports reborrowing (i.e. `decompose!` does not consume its input. Once you're done
   with the borrow, you can reuse the original tuple).
- Produces (admittedly pretty ugly) errors at compile time if the tuple cannot be decomposed.
- Supports borrowing mutable, immutable, owned, and smart-pointer wrapped (so long as they implement
  `Borrow`) components.
- Supports `no_std` environments.
- Has zero runtime dependencies.

use rx_core::prelude::*;

#[derive(Debug)]
pub struct Foo(pub i32);

impl From<i32> for Foo {
	fn from(value: i32) -> Self {
		Foo(value)
	}
}

/// The [IntoOperator] calls `into()` to map incoming values to the expected
/// output value, provided `From` is implemented on the downstream type.
/// When `In` and `Out`, as well as `InError` and `OutError`, are the same types,
/// it is equivalent to the `identity` operator and is a no-op.
fn main() {
	let _s = (1..=5)
		.into_observable()
		.map_into()
		.subscribe(PrintObserver::<Foo>::new("into_operator"));
}

use rx_core::prelude::*;

#[derive(Debug)]
pub struct Foo(pub i32);

impl From<i32> for Foo {
	fn from(value: i32) -> Self {
		Foo(value)
	}
}

/// The [IntoOperator] calls `into()` to map incoming values to the expected
/// out value provided `From` is implemented on the downstream type.
/// When both `In` and `Out`, and `InError` and `OutError` types are the same,
/// it's equivalent to the `identity` operator and is a noop.
fn main() {
	let _s = (1..=5)
		.into_observable::<()>()
		.map_into()
		.subscribe(PrintObserver::<Foo, ()>::new("into_operator"), &mut ());
}

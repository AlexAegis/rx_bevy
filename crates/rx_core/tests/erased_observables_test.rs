use rx_core::{ErasedObservables, prelude::just};

#[test]
fn should_be_able_to_convert_a_tuple() {
	let observables = ErasedObservables::from((just(1), just(2), just(3)));
	assert_eq!(observables.len(), 3);
}

#[test]
fn should_be_able_to_convert_an_array_and_mut_deref() {
	let mut observables = ErasedObservables::from([just(1), just(2), just(3)]);
	assert_eq!(observables.as_mut().len(), 3);
}

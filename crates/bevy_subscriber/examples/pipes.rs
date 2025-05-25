use std::marker::PhantomData;

use bevy_subscriber::{
	observables::{Observable, OfObservable},
	observers::PrintObserver,
	operators::{MapOperator, OperatorSubscribe},
};

fn main() {
	println!("SIGNAL");

	let observable = OfObservable::<i32>::new(12);
	let mapper = |n: i32| -> String {
		return n.to_string();
	};
	let map = MapOperator {
		source_observable: Some(observable),
		transform: mapper,
		phantom_in: PhantomData,
		phantom_out: PhantomData,
	};

	let observer = PrintObserver::<String>::new("hello".to_string());

	map.internal_subscribe(observer);
}

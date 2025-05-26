use bevy_subscriber::prelude::*;

/// This is NOT how you would normally construct a chain of operators,
/// but this is what happens under the hood when you pipe operators.
pub fn main() {
	let observable = OfObservable::<i32>::new(12);

	let map = MapOperator::new_with_source(observable, |n: i32| -> i32 {
		return n * 2;
	});

	let map_2 = MapOperator::new_with_source(map, |n: i32| -> String {
		return n.to_string();
	});

	let observer = FnObserver::new(|next| println!("{next}"));

	map_2.subscribe(observer);
}

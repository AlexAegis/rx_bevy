use bevy_subscriber::{
	observables::{Observable, OfObservable},
	observers::PrintObserver,
	operators::MapOperator,
};

fn main() {
	println!("SIGNAL");

	let observable = OfObservable::<i32>::new(12);

	let map = MapOperator::new(observable, |n: i32| -> i32 {
		return n * 2;
	});

	let map_2 = MapOperator::new(map, |n: i32| -> String {
		return n.to_string();
	});

	let observer = PrintObserver::<String>::new("hello".to_string());

	map_2.internal_subscribe(observer);
}

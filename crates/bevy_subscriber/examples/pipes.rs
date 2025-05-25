use bevy_subscriber::{
	observables::{Observable, ObservableWithOperators, OfObservable},
	observers::PrintObserver,
	operators::MapOperator,
};

fn main() {
	println!("SIGNAL");

	let observable = OfObservable::<i32>::new(12);
	let pipe = observable
		.map(|n: i32| -> i32 {
			return n * 2;
		})
		.map(|n: i32| -> String {
			return n.to_string();
		});

	let observer = PrintObserver::<String>::new("hello".to_string());

	pipe.subscribe(observer);
}

fn pipe_single() {
	println!("SIGNAL");

	let observable = OfObservable::<i32>::new(12);
	let pipe = observable
		.pipe(MapOperator::new(|n: i32| -> i32 {
			return n * 2;
		}))
		.pipe(MapOperator::new(|n: i32| -> String {
			return n.to_string();
		}));

	let observer = PrintObserver::<String>::new("hello".to_string());

	pipe.subscribe(observer);
}

pub fn manual() {
	let observable = OfObservable::<i32>::new(12);

	let map = MapOperator::new_with_source(observable, |n: i32| -> i32 {
		return n * 2;
	});

	let map_2 = MapOperator::new_with_source(map, |n: i32| -> String {
		return n.to_string();
	});

	let observer = PrintObserver::<String>::new("hello".to_string());

	map_2.subscribe(observer);
}

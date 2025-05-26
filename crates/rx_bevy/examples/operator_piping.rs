use rx_bevy::prelude::*;

/// Generic operators can be passed into the pipe function
fn main() {
	let observable = OfObservable::<i32>::new(12);
	let pipe = observable
		.pipe(MapOperator::new(|n: i32| -> i32 {
			return n * 2;
		}))
		.pipe(MapOperator::new(|n: i32| -> String {
			return n.to_string();
		}));

	let observer = FnObserver::new(|next| println!("{next}"));

	pipe.subscribe(observer);
}

use bevy_subscriber::{
	observables::{Observable, OfObservable},
	observers::PrintObserver,
};

fn main() {
	println!("SIGNAL");

	let mut observable = OfObservable::<i32>::new(12);
	let observer = PrintObserver::<i32>::new();
	observable.subscribe(observer);
}

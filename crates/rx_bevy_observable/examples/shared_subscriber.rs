use rx_bevy::prelude::*;

fn main() {
	let observer = PrintObserver::<i32>::new("shared_subscriber");

	let mut shared_subscriber = SharedSubscriber::new(ObserverSubscriber::new(observer));

	shared_subscriber.next(1);
}

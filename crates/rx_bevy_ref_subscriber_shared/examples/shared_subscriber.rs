use rx_bevy::prelude::*;
use rx_bevy_ref_subscriber_shared::SharedSubscriber;

fn main() {
	let observer = PrintObserver::<i32>::new("shared_subscriber");

	let mut shared_subscriber = SharedSubscriber::new(observer);

	shared_subscriber.next(1);
}

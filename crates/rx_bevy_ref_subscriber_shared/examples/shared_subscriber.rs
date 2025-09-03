use rx_bevy::prelude::*;
use rx_bevy_ref_subscriber_shared::SharedSubscriber;
use rx_bevy_subscriber_observer::ObserverSubscriber;

fn main() {
	let observer = PrintObserver::<i32>::new("shared_subscriber");

	let mut shared_subscriber = SharedSubscriber::new(ObserverSubscriber::new(observer));

	shared_subscriber.next(1);
}

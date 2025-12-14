use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriptionLike};

use crate::Multicast;

#[test]
fn should_close_its_subscriptions_once_closed_and_the_drained_subscribers_unsubscribed() {
	let mut multicast = Multicast::<usize, &'static str>::default();

	let subscription_1 = multicast.subscribe(MockObserver::default());
	let drained = multicast.close_and_drain();

	assert!(
		multicast.is_closed(),
		"should be closed after closing and draining"
	);

	assert_eq!(drained.len(), 1, "should've return the one subscriber");

	assert!(
		!subscription_1.is_closed(),
		"the subscription is closed too early"
	);

	for mut shared_multicast_subscriber in drained.into_iter() {
		shared_multicast_subscriber.unsubscribe();
	}

	assert!(subscription_1.is_closed(), "the subscription is not closed");
}

#[test]
fn should_close_its_subscriptions_once_unsubscribed() {
	let mut multicast = Multicast::<usize, &'static str>::default();

	let subscription_1 = multicast.subscribe(MockObserver::default());
	multicast.unsubscribe();

	assert!(multicast.is_closed(), "should be closed after unsubscribe");

	assert!(subscription_1.is_closed(), "the subscription is not closed");
}

#[test]
fn repeatedly_closing_a_multicast_should_do_nothing_even_with_an_attempted_subscribe_after_close() {
	let mut multicast = Multicast::<usize, &'static str>::default();

	let _subscription_1 = multicast.subscribe(MockObserver::default());
	let drained = multicast.close_and_drain();

	assert_eq!(
		drained.len(),
		1,
		"should've returned the one subscriber when closed"
	);

	let subscription_2 = multicast.subscribe(MockObserver::default());

	assert!(
		subscription_2.is_closed(),
		"subscribing to a closed multicast should return a pre-closed subscription"
	);

	let drained_again = multicast.close_and_drain();

	assert!(
		drained_again.is_empty(),
		"should not have returned anything when closed for a second time"
	);
}

use std::time::Duration;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Subscriber, SubscriptionContext, Tickable};

use crate::operator::DelayOperatorOptions;

struct Delayed<T> {
	remaining_time: Duration,
	item: Option<T>,
}

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_context(Destination::Context)]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct DelaySubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	options: DelayOperatorOptions,
	buffer: Vec<Delayed<Destination::In>>,
}

impl<Destination> DelaySubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination, options: DelayOperatorOptions) -> Self {
		Self {
			destination,
			options,
			buffer: Vec::new(),
		}
	}
}

impl<Destination> Observer for DelaySubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.buffer.push(Delayed {
			remaining_time: self.options.delay,
			item: Some(next),
		});
		// TODO: With the better scheduler, it will be a task in the task pool. Try it in bevy
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}

impl<Destination> Tickable for DelaySubscriber<Destination>
where
	Destination: Subscriber,
{
	fn tick(
		&mut self,
		tick: rx_core_traits::Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		// SHORTCOMINGS OF THE CURRENT SETUP: the nexted item has no idea of the scheduler and doesn't know when now is, could be stored from the tick but it'd be outdated info
		let mut nexts = Vec::<Destination::In>::new();
		self.buffer.retain_mut(|item| {
			item.remaining_time = item.remaining_time.saturating_sub(tick.delta);

			if item.remaining_time.is_zero() {
				nexts.push(item.item.take().unwrap());
				false
			} else {
				true
			}
		});
		for next in nexts {
			self.destination.next(next, context);
		}

		self.destination.tick(tick, context);
	}
}

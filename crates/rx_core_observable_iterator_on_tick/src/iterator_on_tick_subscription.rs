use std::iter::Peekable;

use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Never, Signal, Subscriber, SubscriptionContext, SubscriptionData, SubscriptionLike,
	TeardownCollection, Tick, Tickable,
};

use crate::observable::OnTickObservableOptions;

#[derive(RxSubscription)]
#[rx_context(Context)]
pub struct OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	Context: SubscriptionContext,
{
	observed_ticks: usize,
	peekable_iterator: Peekable<Iterator::IntoIter>,
	options: OnTickObservableOptions,
	destination:
		Box<dyn Subscriber<In = Iterator::Item, InError = Never, Context = Context> + Send + Sync>,
	teardown: SubscriptionData<Context>,
}

impl<Iterator, Context> OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	Context: SubscriptionContext,
{
	pub fn new(
		mut destination: impl Subscriber<In = Iterator::Item, InError = Never, Context = Context>
		+ 'static,
		iterator: Iterator::IntoIter,
		options: OnTickObservableOptions,
		context: &mut Context::Item<'_, '_>,
	) -> Self {
		let mut peekable_iterator = iterator.peekable();

		if options.start_on_subscribe
			&& let Some(value) = peekable_iterator.next()
		{
			destination.next(value, context);
		}

		OnTickIteratorSubscription {
			observed_ticks: 0,
			peekable_iterator,
			options,
			destination: Box::new(destination),
			teardown: SubscriptionData::default(),
		}
	}
}

impl<Iterator, Context> Tickable for OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	Context: SubscriptionContext,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.observed_ticks += 1;

			if self.options.emit_at_every_nth_tick != 0
				&& self
					.observed_ticks
					.is_multiple_of(self.options.emit_at_every_nth_tick)
				&& let Some(value) = self.peekable_iterator.next()
			{
				self.observed_ticks = 0; // Reset to avoid overflow
				self.destination.next(value, context);
				if self.peekable_iterator.peek().is_none() {
					self.destination.complete(context);
					self.unsubscribe(context);
				}
			}
		}

		self.destination.tick(tick, context);
	}
}

impl<Iterator, Context> SubscriptionLike for OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	Context: SubscriptionContext,
{
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.teardown.is_closed() {
			self.destination.unsubscribe(context);
			self.teardown.unsubscribe(context);
		}
	}
}

impl<Iterator, Context> TeardownCollection for OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	Context: SubscriptionContext,
{
	fn add_teardown(
		&mut self,
		teardown: rx_core_traits::Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.teardown.add_teardown(teardown, context);
	}
}

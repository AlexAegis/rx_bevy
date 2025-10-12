use rx_bevy_core::{
	SignalBound, Subscriber, SubscriptionData, SubscriptionLike, Tick, Tickable,
	context::{SubscriptionContext, WithSubscriptionContext},
};

use crate::OnTickObservableOptions;

pub struct OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	observed_ticks: usize,
	iterator: Iterator::IntoIter,
	options: OnTickObservableOptions,
	destination:
		Box<dyn Subscriber<In = Iterator::Item, InError = (), Context = Context> + Send + Sync>,
	teardown: SubscriptionData<Context>,
}

impl<Iterator, Context> OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	pub fn new(
		mut destination: impl Subscriber<In = Iterator::Item, InError = (), Context = Context> + 'static,
		iterator: Iterator,
		options: OnTickObservableOptions,
		context: &mut Context,
	) -> Self {
		let mut iter = iterator.into_iter();
		if options.start_on_subscribe
			&& let Some(value) = iter.next()
		{
			destination.next(value, context);
		}

		OnTickIteratorSubscription {
			observed_ticks: 0,
			iterator: iter,
			options,
			destination: Box::new(destination),
			teardown: SubscriptionData::default(),
		}
	}
}

impl<Iterator, Context> WithSubscriptionContext for OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Iterator, Context> Tickable for OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	fn tick(&mut self, _tick: Tick, context: &mut Self::Context) {
		self.observed_ticks += 1;

		if self.options.emit_at_every_nth_tick != 0
			&& self
				.observed_ticks
				.is_multiple_of(self.options.emit_at_every_nth_tick)
			&& let Some(value) = self.iterator.next()
		{
			self.destination.next(value, context);
		}
	}
}

impl<Iterator, Context> SubscriptionLike for OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
		self.teardown.unsubscribe(context);
	}

	fn add_teardown(
		&mut self,
		teardown: rx_bevy_core::Teardown<Self::Context>,
		context: &mut Self::Context,
	) {
		self.teardown.add_teardown(teardown, context);
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.destination.get_context_to_unsubscribe_on_drop()
	}
}

impl<Iterator, Context> Drop for OnTickIteratorSubscription<Iterator, Context>
where
	Iterator: IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = self.get_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}

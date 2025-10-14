use std::marker::PhantomData;

use rx_bevy_core::{
	Observer, ObserverInput, SignalBound, SubscriberNotification, SubscriptionLike, Teardown, Tick,
	Tickable,
	context::{SubscriptionContextDropSafety, WithSubscriptionContext},
	prelude::SubscriptionContext,
};

use crate::MockContext;

#[derive(Debug)]
pub struct MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	pub closed: bool,
	_phantom_data: PhantomData<(In, InError, DropSafety)>,
}

impl<In, InError, DropSafety> ObserverInput for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, DropSafety> Observer for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		context.push(SubscriberNotification::Next(next));
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		context.push(SubscriberNotification::Error(error));
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		context.push(SubscriberNotification::Complete);
		self.unsubscribe(context);
	}
}

impl<In, InError, DropSafety> Tickable for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		context.push(SubscriberNotification::Tick(tick));
	}
}

impl<In, InError, DropSafety> WithSubscriptionContext for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	type Context = MockContext<In, InError, DropSafety>;
}

impl<In, InError, DropSafety> SubscriptionLike for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		self.closed = true;
		context.push(SubscriberNotification::Unsubscribe);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		if self.is_closed() {
			teardown.execute(context);
		}
		context.push(SubscriberNotification::Add(None));
	}
}

impl<In, InError, DropSafety> Default for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	fn default() -> Self {
		Self {
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}

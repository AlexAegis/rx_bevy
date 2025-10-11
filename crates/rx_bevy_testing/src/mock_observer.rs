use std::marker::PhantomData;

use rx_bevy_core::{
	Observer, ObserverInput, SignalBound, SignalContextDropSafety, SubscriberNotification,
	SubscriptionLike, Teardown, Tick, Tickable, WithContext,
};

use crate::MockContext;

#[derive(Debug)]
pub struct MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SignalContextDropSafety,
{
	pub closed: bool,
	_phantom_data: PhantomData<(In, InError, DropSafety)>,
}

impl<In, InError, DropSafety> ObserverInput for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SignalContextDropSafety,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, DropSafety> Observer for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SignalContextDropSafety,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		context.push(SubscriberNotification::Next(next));
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		context.push(SubscriberNotification::Error(error));
	}

	fn complete(&mut self, context: &mut Self::Context) {
		context.push(SubscriberNotification::Complete);
		self.unsubscribe(context);
	}
}

impl<In, InError, DropSafety> Tickable for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SignalContextDropSafety,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		context.push(SubscriberNotification::Tick(tick));
	}
}

impl<In, InError, DropSafety> WithContext for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SignalContextDropSafety,
{
	type Context = MockContext<In, InError, DropSafety>;
}

impl<In, InError, DropSafety> SubscriptionLike for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SignalContextDropSafety,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.closed = true;
		context.push(SubscriberNotification::Unsubscribe);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		if self.is_closed() {
			teardown.execute(context);
		}
		context.push(SubscriberNotification::Add(None));
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		MockContext::default()
	}
}

impl<In, InError, DropSafety> Default for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SignalContextDropSafety,
{
	fn default() -> Self {
		Self {
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}

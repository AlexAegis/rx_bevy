use core::marker::PhantomData;

use rx_core_traits::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, SignalBound,
	SubscriberNotification, SubscriptionClosedFlag, SubscriptionContext,
	SubscriptionContextDropSafety, SubscriptionLike, Teardown, TeardownCollection, Tick, Tickable,
	WithPrimaryCategory, WithSubscriptionContext,
};

use crate::MockContext;

#[derive(Debug)]
pub struct MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	pub closed_flag: SubscriptionClosedFlag,
	_phantom_data: PhantomData<(In, InError, fn(DropSafety))>,
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

impl<In, InError, DropSafety> WithPrimaryCategory for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	/// While this is conceptually an Observer, used as an Observer, for testing
	/// purposes it's marked as a subscriber to not get detached when used as
	/// a destination and be able to track unsubscribe calls.
	type PrimaryCategory = PrimaryCategorySubscriber;
}

/// While this is conceptually an Observer, used as an Observer, for testing
/// purposes it's marked as a subscriber to not get detached when used as
/// a destination and be able to track unsubscribe calls.
impl<In, InError, DropSafety> ObserverUpgradesToSelf for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
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
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		context.push(SubscriberNotification::Next(next));
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		context.push(SubscriberNotification::Error(error));
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		context.push(SubscriberNotification::Complete);
	}
}

impl<In, InError, DropSafety> Tickable for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
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
		*self.closed_flag
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.closed_flag.close();
		context.push(SubscriberNotification::Unsubscribe);
	}
}

impl<In, InError, DropSafety> TeardownCollection for MockObserver<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
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
			closed_flag: false.into(),
			_phantom_data: PhantomData,
		}
	}
}

use core::marker::PhantomData;

use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{
	DropSafeSubscriptionContext, Never, Observer, SignalBound, SubscriberNotification,
	SubscriptionClosedFlag, SubscriptionContext, SubscriptionContextDropSafety, SubscriptionLike,
	Teardown, TeardownCollection, Tick, Tickable,
};

use crate::MockContext;

/// While this is conceptually an Observer, used as an Observer, for testing
/// purposes it behaves like a Subscriber by not being detached on upgrade.
#[derive(RxObserver, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(MockContext<In, InError, DropSafety>)]
#[rx_upgrades_to(self)]
pub struct MockObserver<In, InError = Never, DropSafety = DropSafeSubscriptionContext>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	pub closed_flag: SubscriptionClosedFlag,
	_phantom_data: PhantomData<(In, InError, fn(DropSafety))>,
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

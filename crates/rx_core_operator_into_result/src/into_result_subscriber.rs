use core::marker::PhantomData;

use rx_core_traits::{
	Never, ObservableOutput, Observer, ObserverInput, ObserverUpgradesToSelf,
	PrimaryCategorySubscriber, SignalBound, Subscriber, SubscriptionContext, SubscriptionLike,
	Teardown, TeardownCollection, Tick, Tickable, WithPrimaryCategory, WithSubscriptionContext,
};

pub struct IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> WithSubscriptionContext
	for IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = Destination::Context;
}

impl<In, InError, Destination> WithPrimaryCategory
	for IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<In, InError, Destination> ObserverUpgradesToSelf
	for IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
}

impl<In, InError, Destination> Observer for IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next(Ok(next), context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next(Err(error), context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}

impl<In, InError, Destination> Tickable for IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Destination> SubscriptionLike for IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.unsubscribe(context);
	}
}

impl<In, InError, Destination> TeardownCollection for IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

impl<In, InError, Destination> ObserverInput for IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Destination> ObservableOutput for IntoResultSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber,
{
	type Out = Result<In, InError>;
	type OutError = Never;
}

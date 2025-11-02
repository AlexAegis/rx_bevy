use core::marker::PhantomData;

use rx_core_traits::{
	ObservableOutput, Observer, ObserverInput, SignalBound, Subscriber, SubscriptionContext,
	SubscriptionLike, Teardown, Tick, Tickable, WithSubscriptionContext,
};

#[derive(Debug)]
pub struct TakeSubscriber<In, InError, Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
	count: usize,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> TakeSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination, count: usize) -> Self {
		Self {
			destination,
			count,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> WithSubscriptionContext for TakeSubscriber<In, InError, Destination>
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

impl<In, InError, Destination> Observer for TakeSubscriber<In, InError, Destination>
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
		if !self.is_closed() && self.count > 0 {
			self.count -= 1;
			self.destination.next(next, context);

			if self.count == 0 {
				self.complete(context);
			}
		}
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.destination.error(error, context);
		}
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.destination.complete(context);
		}
	}
}

impl<In, InError, Destination> Tickable for TakeSubscriber<In, InError, Destination>
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

impl<In, InError, Destination> SubscriptionLike for TakeSubscriber<In, InError, Destination>
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

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

impl<In, InError, Destination> ObservableOutput for TakeSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Destination> ObserverInput for TakeSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

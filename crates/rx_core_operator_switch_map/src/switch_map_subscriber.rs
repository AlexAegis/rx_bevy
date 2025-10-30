use std::marker::PhantomData;

use rx_core_subscriber_switch::SwitchSubscriber;
use rx_core_traits::{
	Observable, ObservableOutput, Observer, ObserverInput, SignalBound, Subscriber,
	SubscriptionCollection, SubscriptionContext, SubscriptionLike, Teardown, Tick, Tickable,
	WithSubscriptionContext,
};

pub struct SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	destination: SwitchSubscriber<InnerObservable, Destination>,
	switcher: Switcher,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Switcher, InnerObservable, Destination>
	SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	pub fn new(
		destination: Destination,
		switcher: Switcher,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			destination: SwitchSubscriber::new(destination, context),
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, InnerObservable, Destination> WithSubscriptionContext
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	type Context = Destination::Context;
}

impl<In, InError, Switcher, InnerObservable, Destination> Observer
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next((self.switcher)(next), context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error.into(), context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}

impl<In, InError, Switcher, InnerObservable, Destination> Tickable
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
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

impl<In, InError, Switcher, InnerObservable, Destination> SubscriptionLike
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
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

impl<In, InError, Switcher, InnerObservable, Destination> ObserverInput
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Switcher, InnerObservable, Destination> ObservableOutput
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, Operation, ShareableSubscriber,
	SignalContext, Subscriber, SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};
use rx_bevy_ref_subscriber_switch::SwitchSubscriber;

pub struct SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static + ShareableSubscriber<Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
{
	// TODO: Check if it would be enough to use this in a bevy context by just swapping the SwitchSubscriber impl to an ECS based one.
	destination: SwitchSubscriber<InnerObservable, Destination, Sharer>,
	switcher: Switcher,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination>
	SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Clone + Fn(In) -> InnerObservable,
	Sharer: 'static + ShareableSubscriber<Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	pub fn new(destination: Destination, switcher: Switcher) -> Self {
		Self {
			destination: SwitchSubscriber::new(destination),
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> SignalContext
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static + ShareableSubscriber<Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
{
	type Context = <InnerObservable::Subscription as SignalContext>::Context;
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> Observer
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static + ShareableSubscriber<Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next((self.switcher)(next), context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.error(error.into(), context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
	}

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> SubscriptionLike
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static + ShareableSubscriber<Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> SubscriptionCollection
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static + ShareableSubscriber<Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
	Destination: SubscriptionCollection,
	Sharer::Shared: SubscriptionCollection,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> ObserverInput
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static + ShareableSubscriber<Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> ObservableOutput
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static + ShareableSubscriber<Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> Operation
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static + ShareableSubscriber<Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	type Destination = Destination;
}

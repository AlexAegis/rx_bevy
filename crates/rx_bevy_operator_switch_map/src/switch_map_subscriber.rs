use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, ShareableSubscriber, SignalContext,
	Subscriber, SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};
use rx_bevy_ref_subscriber_switch::SwitchSubscriber;

pub struct SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Destination::Context,
		>,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
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
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Destination::Context,
		>,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
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
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Destination::Context,
		>,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type Context = Sharer::Context;
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> Observer
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Destination::Context,
		>,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
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
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Destination::Context,
		>,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
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
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Destination::Context,
		>,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
	Destination: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
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
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Destination::Context,
		>,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> ObservableOutput
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
	Switcher: Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Destination::Context,
		>,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

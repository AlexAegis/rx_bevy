use std::marker::PhantomData;

use rx_bevy_core::{
	DestinationSharer, Observable, ObservableOutput, Observer, ObserverInput, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick, WithContext,
};
use rx_bevy_ref_subscriber_switch::SwitchSubscriber;

pub struct SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
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
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	pub fn new(destination: Destination, switcher: Switcher) -> Self {
		Self {
			destination: SwitchSubscriber::new(destination),
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> WithContext
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	type Context = Destination::Context;
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> Observer
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next((self.switcher)(next), context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.error(error.into(), context);
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
		self.destination.unsubscribe(context);
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
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
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
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		println!("add teardown switch map sub");
		self.destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.destination.get_context_to_unsubscribe_on_drop()
	}
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> ObserverInput
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Switcher, Sharer, InnerObservable, Destination> ObservableOutput
	for SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = Sharer::Context,
		>,
	Sharer: SubscriptionCollection,
	Sharer::Shared<Destination>: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

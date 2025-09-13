use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, TeardownFn, Tick,
};
use rx_bevy_ref_subscriber_switch::SwitchSubscriber;

pub struct SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	// TODO: Check if it would be enough to use this in a bevy context by just swapping the SwitchSubscriber impl to an ECS based one.
	destination: SwitchSubscriber<InnerObservable, Destination>,
	switcher: Switcher,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Switcher, InnerObservable, Destination>
	SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Clone + Fn(In) -> InnerObservable,
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

impl<In, InError, Switcher, InnerObservable, Destination> SignalContext
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
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

impl<In, InError, Switcher, InnerObservable, Destination> Observer
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
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

impl<In, InError, Switcher, InnerObservable, Destination> SubscriptionLike
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
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
}

impl<In, InError, Switcher, InnerObservable, Destination> SubscriptionCollection
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add<S: 'static + SubscriptionLike<Context = <Self as SignalContext>::Context>>(
		&mut self,
		subscription: S,
		context: &mut Self::Context,
	) {
		self.destination.add(subscription, context);
	}
}

impl<In, InError, Switcher, InnerObservable, Destination> ObserverInput
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
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

impl<In, InError, Switcher, InnerObservable, Destination> ObservableOutput
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
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

impl<In, InError, Switcher, InnerObservable, Destination> Operation
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>
		+ Clone,
{
	type Destination = Destination;

	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		self.destination.read_destination(|shared_subscriber| {
			shared_subscriber.read_destination(|shared_destination| {
				let lock = shared_destination.read().expect("not be poisoned");
				reader(&lock);
			});
		});
	}

	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		self.destination.write_destination(|shared_subscriber| {
			shared_subscriber.write_destination(|shared_destination| {
				let mut lock = shared_destination.write().expect("not be poisoned");
				writer(&mut lock);
			});
		});
	}
}

use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, ObserverInput, Operator, ShareableSubscriber, SignalContext,
	Subscriber, SubscriptionCollection,
};

use crate::SwitchMapSubscriber;

pub struct SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
{
	pub switcher: Switcher,
	pub _phantom_data: PhantomData<(In, InError, Sharer, InnerObservable)>,
}

impl<In, InError, Switcher, Sharer, InnerObservable>
	SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
{
	pub fn new(switcher: Switcher) -> Self {
		Self {
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, Sharer, InnerObservable> Operator
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>
		+ SubscriptionCollection,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
{
	type Context = <Sharer as SignalContext>::Context;
	type Subscriber<Destination>
		= SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Sharer as SignalContext>::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection,
	{
		SwitchMapSubscriber::new(destination, self.switcher.clone())
	}
}

impl<In, InError, Switcher, Sharer, InnerObservable> ObserverInput
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Switcher, Sharer, InnerObservable> ObservableOutput
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

impl<In, InError, Switcher, Sharer, InnerObservable> SignalContext
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
{
	type Context = Sharer::Context;
}

impl<In, InError, Switcher, Sharer, InnerObservable> Clone
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
	InnerObservable: 'static + Observable<Subscription = Sharer>,
{
	fn clone(&self) -> Self {
		Self {
			switcher: self.switcher.clone(),
			_phantom_data: PhantomData,
		}
	}
}

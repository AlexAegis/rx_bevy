use std::marker::PhantomData;

use rx_bevy_core::{
	DestinationSharer, Observable, ObservableOutput, ObserverInput, Operator, Subscriber,
	SubscriptionCollection, WithContext,
};

use crate::SwitchMapSubscriber;

pub struct SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	InnerObservable: 'static + Observable,
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
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	InnerObservable: 'static + Observable,
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
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>
		+ SubscriptionCollection,
{
	type Context = <Sharer as WithContext>::Context;
	type Subscriber<Destination>
		= SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Sharer as WithContext>::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
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
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	InnerObservable: 'static + Observable,
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
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	InnerObservable: 'static + Observable,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

impl<In, InError, Switcher, Sharer, InnerObservable> WithContext
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	InnerObservable: 'static + Observable,
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
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	InnerObservable: 'static + Observable,
{
	fn clone(&self) -> Self {
		Self {
			switcher: self.switcher.clone(),
			_phantom_data: PhantomData,
		}
	}
}

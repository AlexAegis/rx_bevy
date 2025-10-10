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
		context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		SwitchMapSubscriber::new(destination, self.switcher.clone(), context)
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

#[cfg(test)]
mod test {

	use rx_bevy::prelude::*;
	use rx_bevy_core::{ErasedArcSubscriber, use_sharer};
	use rx_bevy_testing::prelude::*;

	#[test]
	fn t() {
		let mut context = MockContext::default();
		let mock_destination = MockObserver::<i32, (), DropSafeSignalContext>::default();

		let mut source = (1..=2)
			.into_observable::<MockContext<_, _, _>>()
			.switch_map(
				|_| (10..=12).into_observable(),
				use_sharer::<ErasedArcSubscriber<_, _, _>>(),
			);
		let _subscription = source.subscribe(mock_destination, &mut context);
		println!("{context:?}");
		assert!(
			context.nothing_happened_after_closed(),
			"something happened after unsubscribe"
		);
		assert_eq!(context.all_observed_values(), vec![10, 11, 12, 10, 11, 12]);
	}
}

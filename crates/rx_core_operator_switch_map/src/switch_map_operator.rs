use std::marker::PhantomData;

use rx_core_traits::{
	Observable, ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber,
	context::WithSubscriptionContext, prelude::SubscriptionContext,
};

use crate::SwitchMapSubscriber;

pub struct SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: 'static + Observable + Send + Sync,
{
	pub switcher: Switcher,
	pub _phantom_data: PhantomData<(In, InError, InnerObservable)>,
}

impl<In, InError, Switcher, InnerObservable>
	SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: 'static + Observable + Send + Sync,
{
	pub fn new(switcher: Switcher) -> Self {
		Self {
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, InnerObservable> Operator
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
{
	type Context = InnerObservable::Context;

	type Subscriber<Destination>
		= SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		SwitchMapSubscriber::new(destination, self.switcher.clone(), context)
	}
}

impl<In, InError, Switcher, InnerObservable> ObserverInput
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: 'static + Observable + Send + Sync,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Switcher, InnerObservable> ObservableOutput
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: 'static + Observable + Send + Sync,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

impl<In, InError, Switcher, InnerObservable> WithSubscriptionContext
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: 'static + Observable + Send + Sync,
{
	type Context = InnerObservable::Context;
}

impl<In, InError, Switcher, InnerObservable> Clone
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: 'static + Fn(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: 'static + Observable + Send + Sync,
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

	use rx_core::prelude::*;
	use rx_core_testing::prelude::*;

	#[test]
	fn t() {
		let mut context = MockContext::default();
		let mock_destination = MockObserver::<i32, (), DropSafeSubscriptionContext>::default();

		let mut source = (1..=2)
			.into_observable::<MockContext<_, _, _>>()
			.switch_map(|_| (10..=12).into_observable::<()>());
		let _subscription = source.subscribe(mock_destination, &mut context);
		println!("{context:?}");
		assert!(
			context.nothing_happened_after_closed(),
			"something happened after unsubscribe"
		);
		assert_eq!(context.all_observed_values(), vec![10, 11, 12, 10, 11, 12]);
	}
}

use std::marker::PhantomData;

use rx_bevy_core::{Observable, ObservableOutput, SharedSubscriber, Subscriber};

use crate::{FixedSubscriberObservable, IntoFixedSubscriberObservable};

// TODO:This entire file and future macros into it's own crate observable_collections
pub struct ObservableBundle2<Observable0, Observable1>(pub Observable0, pub Observable1)
where
	Observable0: Observable,
	Observable1: Observable;

/// A bundle of observables whose output can be converted to a common output type
/// using `Into`.
pub struct IntoBoundObservableBundle2<Out, OutError, Observable0, Observable1>
where
	Out: 'static,
	OutError: 'static,
	Observable0::Out: 'static + Into<Out>,
	Observable0::OutError: 'static + Into<OutError>,
	Observable0: 'static + Observable,
	Observable1::Out: 'static + Into<Out>,
	Observable1::OutError: 'static + Into<OutError>,
	Observable1: 'static + Observable,
{
	pub o0: Observable0,
	pub o1: Observable1,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError, Observable0, Observable1> ObservableOutput
	for IntoBoundObservableBundle2<Out, OutError, Observable0, Observable1>
where
	Out: 'static,
	OutError: 'static,
	Observable0::Out: 'static + Into<Out>,
	Observable0::OutError: 'static + Into<OutError>,
	Observable0: 'static + Observable,
	Observable1::Out: 'static + Into<Out>,
	Observable1::OutError: 'static + Into<OutError>,
	Observable1: 'static + Observable,
{
	type Out = Out;
	type OutError = OutError;
}

impl<Out, OutError, Observable0, Observable1> From<ObservableBundle2<Observable0, Observable1>>
	for IntoBoundObservableBundle2<Out, OutError, Observable0, Observable1>
where
	Out: 'static,
	OutError: 'static,
	Observable0::Out: 'static + Into<Out>,
	Observable0::OutError: 'static + Into<OutError>,
	Observable0: 'static + Observable,
	Observable1::Out: 'static + Into<Out>,
	Observable1::OutError: 'static + Into<OutError>,
	Observable1: 'static + Observable,
{
	fn from(value: ObservableBundle2<Observable0, Observable1>) -> Self {
		Self {
			o0: value.0,
			o1: value.1,
			_phantom_data: PhantomData,
		}
	}
}

/// For Observable bundles that share the same outputs without having to convert
/// them, easier to implement with, less capable
pub struct CommonBoundObservableBundle2<Observable0, Observable1>
where
	Observable0::Out: 'static,
	Observable0::OutError: 'static,
	Observable0: 'static + Observable,
	Observable1: 'static + Observable<Out = Observable0::Out, OutError = Observable0::OutError>,
{
	pub o0: Observable0,
	pub o1: Observable1,
}

impl<Observable0, Observable1> ObservableOutput
	for CommonBoundObservableBundle2<Observable0, Observable1>
where
	Observable0::Out: 'static,
	Observable0::OutError: 'static,
	Observable0: 'static + Observable,
	Observable1: 'static + Observable<Out = Observable0::Out, OutError = Observable0::OutError>,
{
	type Out = Observable0::Out;
	type OutError = Observable0::OutError;
}

impl<Observable0, Observable1> From<ObservableBundle2<Observable0, Observable1>>
	for CommonBoundObservableBundle2<Observable0, Observable1>
where
	Observable0::Out: 'static,
	Observable0::OutError: 'static,
	Observable0: 'static + Observable,
	Observable1: 'static + Observable<Out = Observable0::Out, OutError = Observable0::OutError>,
{
	fn from(value: ObservableBundle2<Observable0, Observable1>) -> Self {
		Self {
			o0: value.0,
			o1: value.1,
		}
	}
}

pub struct ObservableCollection<Out, OutError, Destination>
where
	Out: 'static,
	OutError: 'static,
	Destination: 'static + Subscriber<In = Out, InError = OutError>,
{
	pub destination: SharedSubscriber<Destination>,
	pub observables: Vec<
		Box<
			dyn FixedSubscriberObservable<
					SharedSubscriber<Destination>,
					Out = Out,
					OutError = OutError,
				>,
		>,
	>,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError, Destination> ObservableOutput
	for ObservableCollection<Out, OutError, Destination>
where
	Out: 'static,
	OutError: 'static,
	Destination: 'static + Subscriber<In = Out, InError = OutError>,
{
	type Out = Out;
	type OutError = OutError;
}

impl<Observable0, Observable1> CommonBoundObservableBundle2<Observable0, Observable1>
where
	Observable0::Out: 'static,
	Observable0::OutError: 'static,
	Observable0: 'static + IntoFixedSubscriberObservable + Observable,
	Observable1: 'static
		+ IntoFixedSubscriberObservable
		+ Observable<Out = Observable0::Out, OutError = Observable0::OutError>,
{
	pub fn into_observable_collection<Destination>(
		self,
		destination: Destination,
	) -> ObservableCollection<Observable0::Out, Observable0::OutError, Destination>
	where
		Destination: 'static + Subscriber<In = Observable0::Out, InError = Observable0::OutError>,
		Observable0::FixedSubscriberObservable<SharedSubscriber<Destination>>:
			FixedSubscriberObservable<
					Destination,
					Out = Observable0::Out,
					OutError = Observable0::OutError,
				>,
		Observable1::FixedSubscriberObservable<SharedSubscriber<Destination>>:
			FixedSubscriberObservable<
					Destination,
					Out = Observable0::Out,
					OutError = Observable0::OutError,
				>,
	{
		let shared_destination = SharedSubscriber::new(destination);
		ObservableCollection {
			destination: shared_destination.clone(),
			observables: vec![
				Box::new(
					self.o0
						.into_fixed_subscriber_observable::<SharedSubscriber<Destination>>(
							shared_destination.clone(),
						),
				),
				Box::new(
					self.o1
						.into_fixed_subscriber_observable::<SharedSubscriber<Destination>>(
							shared_destination.clone(),
						),
				),
			],
			_phantom_data: PhantomData,
		}
	}
}

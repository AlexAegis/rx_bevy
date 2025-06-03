use std::marker::PhantomData;

use rx_bevy_observable::{Observer, ObserverConnector};
use rx_bevy_operator::{ForwardObserver, Operator, OperatorCallback};

// TODO: Actually implement it, this just a copy of the map operator now
pub struct SwitchMapOperator<In, Out, F, Error> {
	pub mapper: F,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<In, Out, Mapper, Error> Operator for SwitchMapOperator<In, Out, Mapper, Error>
where
	Mapper: OperatorCallback<In, Out> + Clone,
{
	type In = In;
	type Out = Out;
	type InError = Error;
	type OutError = Error;

	type InternalSubscriber = Self;

	fn operator_subscribe<
		Destination: 'static + Observer<In = Self::Out, Error = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> ForwardObserver<Self::InternalSubscriber, Destination> {
		ForwardObserver::new(self.clone(), destination)
	}
}

impl<In, Out, F, Error> ObserverConnector for SwitchMapOperator<In, Out, F, Error>
where
	F: Fn(In) -> Out,
{
	type In = In;
	type Out = Out;
	type InError = Error;
	type OutError = Error;

	#[inline]
	fn next_forward<Destination: Observer<In = Out>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		let mapped = (self.mapper)(next);
		destination.next(mapped);
	}

	#[inline]
	fn error_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		destination.error(error);
	}
}

impl<In, Out, F, Error> SwitchMapOperator<In, Out, F, Error> {
	pub fn new(transform: F) -> Self {
		Self {
			mapper: transform,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Out, F, Error> Clone for SwitchMapOperator<In, Out, F, Error>
where
	F: Clone,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}

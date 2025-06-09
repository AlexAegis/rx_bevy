use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, Observer, Subscriber};
use rx_bevy_operator::Operator;

pub struct SwitchMapOperator<In, Out, F, Error> {
	pub mapper: F,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<In, Out, Mapper, Error> Operator for SwitchMapOperator<In, Out, Mapper, Error>
where
	Mapper: Clone + Fn(In) -> Out,
{
	type Fw = SwitchMapForwarder<In, Out, Mapper, Error>;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<In = <Self::Fw as Forwarder>::Out, Error = <Self::Fw as Forwarder>::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Fw, Destination> {
		Subscriber::new(destination, SwitchMapForwarder::new(self.mapper.clone()))
	}
}

pub struct SwitchMapForwarder<In, Out, F, Error> {
	pub mapper: F,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<In, Out, F, Error> SwitchMapForwarder<In, Out, F, Error> {
	pub fn new(mapper: F) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Out, F, Error> Forwarder for SwitchMapForwarder<In, Out, F, Error>
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

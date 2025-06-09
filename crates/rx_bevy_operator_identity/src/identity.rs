use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, Observer, Subscriber};
use rx_bevy_operator::Operator;

#[derive(Debug)]
pub struct IdentityOperator<In, Error> {
	_phantom_data: PhantomData<(In, Error)>,
}

impl<In, Error> Operator for IdentityOperator<In, Error> {
	type Fw = Self;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<In = <Self::Fw as Forwarder>::Out, Error = <Self::Fw as Forwarder>::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Fw, Destination> {
		Subscriber::new(destination, self.clone())
	}
}

impl<In, Error> Forwarder for IdentityOperator<In, Error> {
	type In = In;
	type Out = In;
	type InError = Error;
	type OutError = Error;

	#[inline]
	fn next_forward<Destination: Observer<In = In>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		destination.next(next);
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

impl<In, Error> IdentityOperator<In, Error> {
	pub fn new() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Error> Clone for IdentityOperator<In, Error> {
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

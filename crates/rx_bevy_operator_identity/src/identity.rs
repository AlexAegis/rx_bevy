use std::marker::PhantomData;

use rx_bevy_observable::{
	Forwarder, ObservableOutput, Observer, ObserverInput, Operator, Subscriber,
};

#[derive(Debug)]
pub struct IdentityOperator<In, InError> {
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Operator for IdentityOperator<In, InError> {
	type Fw = Self;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self::Fw as ObservableOutput>::Out,
				InError = <Self::Fw as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Fw, Destination> {
		Subscriber::new(destination, self.clone())
	}
}

impl<In, InError> ObservableOutput for IdentityOperator<In, InError> {
	type Out = In;
	type OutError = InError;
}

impl<In, InError> ObserverInput for IdentityOperator<In, InError> {
	type In = In;
	type InError = InError;
}

impl<In, InError> Forwarder for IdentityOperator<In, InError> {
	#[inline]
	fn next_forward<Destination: Observer<In = In>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		destination.next(next);
	}

	#[inline]
	fn error_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		destination.error(error);
	}
}

impl<In, InError> IdentityOperator<In, InError> {
	pub fn new() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Clone for IdentityOperator<In, InError> {
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

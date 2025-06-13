use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, ObservableOutput, Observer, ObserverInput, Operator};

#[derive(Debug)]
pub struct IdentityOperator<In, InError> {
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Operator for IdentityOperator<In, InError> {
	type Fw = IdentityForwarder<In, InError>;

	#[inline]
	fn create_instance(&self) -> Self::Fw {
		Self::Fw::default()
	}
}

impl<In, InError> Default for IdentityOperator<In, InError> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

#[derive(Debug)]
pub struct IdentityForwarder<In, InError> {
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Default for IdentityForwarder<In, InError> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ObservableOutput for IdentityForwarder<In, InError> {
	type Out = In;
	type OutError = InError;
}

impl<In, InError> ObserverInput for IdentityForwarder<In, InError> {
	type In = In;
	type InError = InError;
}

impl<In, InError> Forwarder for IdentityForwarder<In, InError> {
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

impl<In, InError> Clone for IdentityOperator<In, InError> {
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

use std::{marker::PhantomData, ops::Sub};

use crate::{ObservableOutput, Observer, ObserverInput, SharedObserver, Subscriber};

pub struct ObserverWrapper<D>
where
	D: Observer,
{
	destination: D,
}

impl<D> ObserverInput for ObserverWrapper<D>
where
	D: Observer,
{
	type In = D::In;
	type InError = D::InError;
}

impl<D> Observer for ObserverWrapper<D>
where
	D: Observer,
{
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	fn complete(&mut self) {
		self.destination.complete();
	}
}
/*
pub trait RootForwarder<D>: ObserverInput + ObservableOutput {
	fn next_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: &mut D,
	);

	fn error_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut D,
	);

	fn complete_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: &mut D,
	);
}
*/

pub struct ForwarderBridge<Fw, Destination>
where
	Fw: Forwarder,
	Destination: Observer,
{
	forwarder: Fw,
	_phantom_data: PhantomData<Destination>,
}

impl<Fw, Destination> ForwarderBridge<Fw, Destination>
where
	Fw: Forwarder,
	Destination: Observer,
{
	pub fn new(forwarder: Fw) -> Self {
		Self {
			forwarder,
			_phantom_data: PhantomData,
		}
	}
}

impl<Fw, Destination> ObservableOutput for ForwarderBridge<Fw, Destination>
where
	Fw: Forwarder,
	Destination: Observer,
{
	type Out = Fw::Out;
	type OutError = Fw::OutError;
}

impl<Fw, Destination> ObserverInput for ForwarderBridge<Fw, Destination>
where
	Fw: Forwarder,
	Destination: Observer,
{
	type In = Fw::In;
	type InError = Fw::InError;
}

impl<Fw, D> SubscriberForwarder for ForwarderBridge<Fw, D>
where
	Fw: Forwarder,
	D: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	type Destination = D;

	fn next_forward(&mut self, next: Self::In, destination: &mut Self::Destination) {
		self.forwarder.next_forward(next, destination);
	}

	fn error_forward(&mut self, error: Self::InError, destination: &mut Self::Destination) {
		self.forwarder.error_forward(error, destination);
	}

	fn complete_forward(&mut self, destination: &mut Self::Destination) {
		self.forwarder.complete_forward(destination);
	}
}

pub trait SubscriberForwarder: ObserverInput + ObservableOutput {
	type Destination: Observer<In = Self::Out, InError = Self::OutError>;

	fn next_forward(&mut self, next: Self::In, destination: &mut Self::Destination);

	fn error_forward(&mut self, error: Self::InError, destination: &mut Self::Destination);

	fn complete_forward(&mut self, destination: &mut Self::Destination);
}

pub trait Forwarder: ObserverInput + ObservableOutput {
	fn next_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	);

	fn error_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	);

	#[inline]
	fn complete_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: &mut Destination,
	) {
		destination.complete();
	}
}

pub trait DynForwarder: ObserverInput + ObservableOutput {
	fn next_forward(
		&mut self,
		next: Self::In,
		destination: &mut dyn Observer<In = Self::Out, InError = Self::OutError>,
	);

	fn error_forward(
		&mut self,
		error: Self::InError,
		destination: &mut dyn Observer<In = Self::Out, InError = Self::OutError>,
	);

	#[inline]
	fn complete_forward(
		&mut self,
		destination: &mut dyn Observer<In = Self::Out, InError = Self::OutError>,
	) {
		destination.complete();
	}
}

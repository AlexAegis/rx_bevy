use crate::{ObservableOutput, Observer, ObserverInput};

pub trait Forwarder: ObserverInput + ObservableOutput {
	fn next_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	);

	fn error_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		next: Self::InError,
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
		next: Self::InError,
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

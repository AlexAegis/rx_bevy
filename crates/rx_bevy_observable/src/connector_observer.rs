use crate::Observer;

pub trait ObserverConnector {
	type In;
	type Out;
	type InError;
	type OutError;

	fn next_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	);

	fn error_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	);

	fn complete_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		destination: &mut Destination,
	) {
		destination.complete();
	}
}

pub trait DynObserverConnector {
	type In;
	type Out;
	type InError;
	type OutError;

	fn next_forward(
		&mut self,
		next: Self::In,
		destination: &mut dyn Observer<In = Self::Out, Error = Self::OutError>,
	);

	fn error_forward(
		&mut self,
		error: Self::InError,
		destination: &mut dyn Observer<In = Self::Out, Error = Self::OutError>,
	);

	#[inline]
	fn complete_forward(
		&mut self,
		destination: &mut dyn Observer<In = Self::Out, Error = Self::OutError>,
	) {
		destination.complete();
	}
}

impl<T> ObserverConnector for T
where
	T: DynObserverConnector,
{
	type In = T::In;
	type Out = T::Out;
	type InError = T::InError;
	type OutError = T::OutError;

	#[inline]
	fn next_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		DynObserverConnector::next_forward(self, next, destination);
	}

	#[inline]
	fn error_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		DynObserverConnector::error_forward(self, error, destination);
	}

	#[inline]
	fn complete_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		destination: &mut Destination,
	) {
		DynObserverConnector::complete_forward(self, destination);
	}
}

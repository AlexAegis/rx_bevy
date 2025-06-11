use crate::{Observable, Observer};

/// A lifted forwarded expects you to return an observable instead of a flat value, the flattening will happen later
///
pub trait LiftingForwarder {
	type In;
	type InError;
	type OutObservable: Observable;

	fn next_forward<
		LiftedDestination: Observer<In = Self::OutObservable, Error = <Self::OutObservable as Observable>::Error>,
	>(
		&mut self,
		next: Self::In,
		destination: &mut LiftedDestination,
	);

	fn error_forward<
		LiftedDestination: Observer<In = Self::OutObservable, Error = <Self::OutObservable as Observable>::Error>,
	>(
		&mut self,
		next: Self::InError,
		destination: &mut LiftedDestination,
	);

	#[inline]
	fn complete_forward<
		LiftedDestination: Observer<In = Self::OutObservable, Error = <Self::OutObservable as Observable>::Error>,
	>(
		&mut self,
		destination: &mut LiftedDestination,
	) {
		destination.complete();
	}
}

pub trait Forwarder {
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
		next: Self::InError,
		destination: &mut Destination,
	);

	#[inline]
	fn complete_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		destination: &mut Destination,
	) {
		destination.complete();
	}
}

pub trait DynForwarder {
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
		next: Self::InError,
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

pub struct Subscriber<Fw, Destination>
where
	Fw: Forwarder,
	Destination: Observer,
{
	pub destination: Destination,
	pub forwarder: Fw,
	pub is_closed: bool,
}

impl<Fw, Destination> Subscriber<Fw, Destination>
where
	Fw: Forwarder,
	Destination: Observer,
{
	pub fn new(destination: Destination, forwarder: Fw) -> Self {
		Self {
			destination,
			forwarder,
			is_closed: false,
		}
	}
}

impl<Fw, Destination> Observer for Subscriber<Fw, Destination>
where
	Fw: Forwarder,
	Destination: Observer<In = Fw::Out, Error = Fw::OutError>,
{
	type In = Fw::In;
	type Error = Fw::InError;

	#[inline]
	fn next(&mut self, next: Self::In) {
		if !self.is_closed {
			self.forwarder.next_forward(next, &mut self.destination);
		} else {
			todo!("handle subscriber next notification")
		}
	}

	#[inline]
	fn error(&mut self, error: Self::Error) {
		if !self.is_closed {
			self.forwarder.error_forward(error, &mut self.destination);
		} else {
			todo!("handle subscriber error notification")
		}
	}

	#[inline]
	fn complete(&mut self) {
		if !self.is_closed {
			self.is_closed = true;
			self.forwarder.complete_forward(&mut self.destination);
		} else {
			todo!("handle subscriber complete notification")
		}
	}
}

pub struct LiftedSubscriber<Fw, Destination>
where
	Fw: LiftingForwarder,
	Destination: Observer,
{
	pub destination: Destination,
	pub forwarder: Fw,
	pub is_closed: bool,
}

impl<Fw, Destination> LiftedSubscriber<Fw, Destination>
where
	Fw: LiftingForwarder,
	Destination: Observer,
{
	pub fn new(destination: Destination, forwarder: Fw) -> Self {
		Self {
			destination,
			forwarder,
			is_closed: false,
		}
	}
}

impl<Fw, Destination> Observer for LiftedSubscriber<Fw, Destination>
where
	Fw: LiftingForwarder,
	Destination: Observer<In = Fw::OutObservable, Error = <Fw::OutObservable as Observable>::Error>,
{
	type In = Fw::In;
	type Error = Fw::InError;

	#[inline]
	fn next(&mut self, next: Self::In) {
		if !self.is_closed {
			self.forwarder.next_forward(next, &mut self.destination);
		} else {
			todo!("handle subscriber next notification")
		}
	}

	#[inline]
	fn error(&mut self, error: Self::Error) {
		if !self.is_closed {
			self.forwarder.error_forward(error, &mut self.destination);
		} else {
			todo!("handle subscriber error notification")
		}
	}

	#[inline]
	fn complete(&mut self) {
		if !self.is_closed {
			self.is_closed = true;
			self.forwarder.complete_forward(&mut self.destination);
		} else {
			todo!("handle subscriber complete notification")
		}
	}
}

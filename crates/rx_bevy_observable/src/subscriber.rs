use crate::{Observer, Subscription};
/*
pub trait OwnedForwarder {
	type In;
	type Out;
	type InError;
	type OutError;

	fn next_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: Destination,
	) -> Destination;

	fn error_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		next: Self::InError,
		destination: Destination,
	) -> Destination;

	#[inline]
	fn complete_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		mut destination: Destination,
	) -> Destination {
		destination.complete();
		destination
	}
}
*/
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
			// if let Some(destination) = self.destination.take() {
			// 	let next_destination = self.forwarder.next_forward(next, destination);
			// 	self.destination = Some(next_destination);
			// } else {
			// 	unreachable!("Can't next destination: No Destination!")
			// }
		} else {
			todo!("handle subscriber next notification")
		}
	}

	#[inline]
	fn error(&mut self, error: Self::Error) {
		if !self.is_closed {
			self.forwarder.error_forward(error, &mut self.destination);
			// if let Some(destination) = self.destination.take() {
			// 	let next_destination = self.forwarder.error_forward(error, destination);
			// 	self.destination = Some(next_destination);
			// } else {
			// 	unreachable!("Can't error destination: No Destination!")
			// }
		} else {
			todo!("handle subscriber error notification")
		}
	}

	#[inline]
	fn complete(&mut self) {
		if !self.is_closed {
			self.is_closed = true;
			self.forwarder.complete_forward(&mut self.destination);
			// if let Some(destination) = self.destination.take() {
			// 	let next_destination = self.forwarder.complete_forward(destination);
			// 	self.destination = Some(next_destination);
			// } else {
			// 	unreachable!("Can't complete destination: No Destination!")
			// }
		} else {
			todo!("handle subscriber complete notification")
		}
	}
}

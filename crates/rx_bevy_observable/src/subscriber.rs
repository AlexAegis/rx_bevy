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
impl<Fw, Destination> ObserverInput for Subscriber<Fw, Destination>
where
	Fw: Forwarder,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	type In = Fw::In;
	type InError = Fw::InError;
}

impl<Fw, Destination> Observer for Subscriber<Fw, Destination>
where
	Fw: Forwarder,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if !self.is_closed {
			self.forwarder.next_forward(next, &mut self.destination);
		} else {
			todo!("handle subscriber next notification")
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
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

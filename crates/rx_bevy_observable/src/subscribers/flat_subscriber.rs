use crate::{
	Observable, ObservableOutput, Observer, ObserverInput, SharedObserver, SubscriberForwarder,
};

pub struct SharedSubscriber<Fw, Destination>
where
	Fw: SubscriberForwarder,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	pub destination: SharedObserver<Destination>,
	pub forwarder: Fw,
	pub is_complete: bool,
}

impl<Fw, Destination> SharedSubscriber<Fw, Destination>
where
	Fw: SubscriberForwarder<Destination = SharedObserver<Destination>>,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	pub fn new(destination: Destination, forwarder: Fw) -> Self {
		Self {
			destination: SharedObserver::new(destination),
			forwarder,
			is_complete: false,
		}
	}
}

impl<Fw, Destination> ObserverInput for SharedSubscriber<Fw, Destination>
where
	Fw: SubscriberForwarder<Destination = SharedObserver<Destination>>,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	type In = Fw::In;
	type InError = Fw::InError;
}

impl<Fw, Destination> ObservableOutput for SharedSubscriber<Fw, Destination>
where
	Fw: SubscriberForwarder<Destination = SharedObserver<Destination>>,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	type Out = Fw::Out;
	type OutError = Fw::OutError;
}

impl<Fw, Destination> Observer for SharedSubscriber<Fw, Destination>
where
	Fw: SubscriberForwarder<Destination = SharedObserver<Destination>>,
	Destination: 'static + Observer<In = Fw::Out, InError = Fw::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if !self.is_complete {
			self.forwarder.next_forward(next, &mut self.destination);
		} else {
			todo!("handle subscriber next notification")
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		if !self.is_complete {
			self.forwarder.error_forward(error, &mut self.destination);
		} else {
			todo!("handle subscriber error notification")
		}
	}

	#[inline]
	fn complete(&mut self) {
		if !self.is_complete {
			self.is_complete = true;
			self.forwarder.complete_forward(&mut self.destination);
		} else {
			todo!("handle subscriber complete notification")
		}
	}
}

pub trait SharedForwarder: ObserverInput + ObservableOutput {
	fn next_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: &mut SharedObserver<Destination>,
	);

	fn error_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut SharedObserver<Destination>,
	);

	#[inline]
	fn complete_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: &mut SharedObserver<Destination>,
	) {
		destination.complete();
	}
}

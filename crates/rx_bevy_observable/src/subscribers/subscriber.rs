use crate::{
	Forwarder, ObservableOutput, Observer, ObserverInput, SharedObserver, SharedSubscriber,
	SubscriberForwarder, subscribers::subscriber,
};

pub struct Subscriber<Fw, Destination>
where
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
	Fw: SubscriberForwarder,
{
	pub destination: Destination,
	pub forwarder: Fw,
	pub is_closed: bool,
}

impl<Fw, Destination> Subscriber<Fw, Destination>
where
	Fw: SubscriberForwarder,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
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
	Fw: SubscriberForwarder,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	type In = Fw::In;
	type InError = Fw::InError;
}

impl<Fw, Destination> Observer for Subscriber<Fw, Destination>
where
	Fw: SubscriberForwarder<Destination = Destination>,
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
/*
pub enum SuperSubscriber<Fw, Destination>
where
	Fw: SubscriberForwarder,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	Flat(SharedSubscriber<Fw, Destination>),
	Regular(Subscriber<Fw, Destination>),
}

impl<Fw, Destination> ObserverInput for SuperSubscriber<Fw, Destination>
where
	Fw: SubscriberForwarder,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	type In = Fw::In;
	type InError = Fw::InError;
}

impl<Fw, Destination> Observer for SuperSubscriber<Fw, Destination>
where
	Fw: SubscriberForwarder,
	Destination: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		match self {
			Self::Regular(subscriber) => subscriber.next(next),
			Self::Flat(subscriber) => subscriber.next(next),
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		match self {
			Self::Regular(subscriber) => subscriber.error(error),
			Self::Flat(subscriber) => subscriber.error(error),
		}
	}

	#[inline]
	fn complete(&mut self) {
		match self {
			Self::Regular(subscriber) => subscriber.complete(),
			Self::Flat(subscriber) => subscriber.complete(),
		}
	}
}
*/

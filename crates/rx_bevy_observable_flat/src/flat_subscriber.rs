use rx_bevy_observable::{Observable, Observer};
use rx_bevy_observer_shared::SharedObserver;

pub trait FlatForwarder {
	type InObservable: Observable;
	type InError;

	fn next_forward<
		Destination: 'static
			+ Observer<
				In = <Self::InObservable as Observable>::Out,
				Error = <Self::InObservable as Observable>::Error,
			>,
	>(
		&mut self,
		next: Self::InObservable,
		destination: &mut SharedObserver<Destination>,
	);

	fn error_forward<
		Destination: 'static
			+ Observer<
				In = <Self::InObservable as Observable>::Out,
				Error = <Self::InObservable as Observable>::Error,
			>,
	>(
		&mut self,
		error: Self::InError,
		destination: &mut SharedObserver<Destination>,
	);

	#[inline]
	fn complete_forward<
		Destination: 'static
			+ Observer<
				In = <Self::InObservable as Observable>::Out,
				Error = <Self::InObservable as Observable>::Error,
			>,
	>(
		&mut self,
		destination: &mut SharedObserver<Destination>,
	) {
		destination.complete();
	}
}

pub struct FlatSubscriber<Fw, Destination>
where
	Fw: FlatForwarder,
	Destination: Observer,
{
	pub destination: SharedObserver<Destination>,
	pub forwarder: Fw,
	pub is_closed: bool,
}

impl<Fw, Destination> FlatSubscriber<Fw, Destination>
where
	Fw: FlatForwarder,
	Destination: Observer<
			In = <Fw::InObservable as Observable>::Out,
			Error = <Fw::InObservable as Observable>::Error,
		>,
{
	pub fn new(destination: Destination, forwarder: Fw) -> Self {
		Self {
			destination: SharedObserver::new(destination),
			forwarder,
			is_closed: false,
		}
	}
}

impl<Fw, Destination> Observer for FlatSubscriber<Fw, Destination>
where
	Fw: FlatForwarder,
	Destination: 'static
		+ Observer<
			In = <Fw::InObservable as Observable>::Out,
			Error = <Fw::InObservable as Observable>::Error,
		>,
{
	type In = Fw::InObservable;
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

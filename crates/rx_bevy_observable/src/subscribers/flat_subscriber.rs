use crate::{Observable, ObservableOutput, Observer, ObserverInput, SharedObserver};

pub trait ForwardFlattener {
	type InObservable: Observable;
	type InError;

	fn flatten_next<
		Destination: 'static
			+ Observer<
				In = <Self::InObservable as ObservableOutput>::Out,
				InError = <Self::InObservable as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		next: Self::InObservable,
		destination: &mut SharedObserver<Destination>,
	);

	fn error_forward<
		Destination: 'static
			+ Observer<
				In = <Self::InObservable as ObservableOutput>::Out,
				InError = <Self::InObservable as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	);

	#[inline]
	fn complete_forward<
		Destination: 'static
			+ Observer<
				In = <Self::InObservable as ObservableOutput>::Out,
				InError = <Self::InObservable as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: &mut Destination,
	) {
		destination.complete();
	}
}

pub struct FlatSubscriber<Fw, Destination>
where
	Fw: ForwardFlattener,
	Destination: Observer,
{
	pub destination: SharedObserver<Destination>,
	pub forwarder: Fw,
	pub is_closed: bool,
}

impl<Fw, Destination> FlatSubscriber<Fw, Destination>
where
	Fw: ForwardFlattener,
	Destination: Observer<
			In = <Fw::InObservable as ObservableOutput>::Out,
			InError = <Fw::InObservable as ObservableOutput>::OutError,
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

impl<Fw, Destination> ObserverInput for FlatSubscriber<Fw, Destination>
where
	Fw: ForwardFlattener,
	Destination: 'static
		+ Observer<
			In = <Fw::InObservable as ObservableOutput>::Out,
			InError = <Fw::InObservable as ObservableOutput>::OutError,
		>,
{
	type In = Fw::InObservable;
	type InError = Fw::InError;
}

impl<Fw, Destination> Observer for FlatSubscriber<Fw, Destination>
where
	Fw: ForwardFlattener,
	Destination: 'static
		+ Observer<
			In = <Fw::InObservable as ObservableOutput>::Out,
			InError = <Fw::InObservable as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if !self.is_closed {
			self.forwarder.flatten_next(next, &mut self.destination);
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

use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, Observer, Operator, Subscriber};

#[derive(Debug)]
pub struct TapOperator<In, Callback, Error>
where
	Callback: for<'a> Fn(&'a In),
{
	callback: Callback,
	_phantom_data: PhantomData<(In, Error)>,
}

impl<In, Callback, Error> Operator for TapOperator<In, Callback, Error>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type Fw = Self;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<In = <Self::Fw as Forwarder>::Out, Error = <Self::Fw as Forwarder>::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Fw, Destination> {
		Subscriber::new(destination, self.clone())
	}
}

impl<In, Callback, Error> Forwarder for TapOperator<In, Callback, Error>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type In = In;
	type Out = In;
	type InError = Error;
	type OutError = Error;

	#[inline]
	fn next_forward<Destination: Observer<In = In>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		(self.callback)(&next);
		destination.next(next);
	}

	#[inline]
	fn error_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		destination.error(error);
	}
}

impl<In, Callback, Error> TapOperator<In, Callback, Error>
where
	Callback: for<'a> Fn(&'a In),
{
	pub fn new(callback: Callback) -> Self {
		Self {
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Callback, Error> Clone for TapOperator<In, Callback, Error>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data: PhantomData,
		}
	}
}

use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, Observer, Subscriber};
use rx_bevy_operator::Operator;

pub struct FilterOperator<T, Filter, Error> {
	pub filter: Filter,
	pub _phantom_data: PhantomData<(T, Error)>,
}

impl<T, Filter, Error> Operator for FilterOperator<T, Filter, Error>
where
	Filter: Clone + for<'a> Fn(&'a T) -> bool,
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

impl<T, Filter, Error> Forwarder for FilterOperator<T, Filter, Error>
where
	Filter: for<'a> Fn(&'a T) -> bool,
{
	type In = T;
	type Out = T;
	type InError = Error;
	type OutError = Error;

	#[inline]
	fn next_forward<Destination: Observer<In = T>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		if (self.filter)(&next) {
			destination.next(next);
		}
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

impl<T, F, Error> Clone for FilterOperator<T, F, Error>
where
	F: Clone,
{
	fn clone(&self) -> Self {
		Self {
			filter: self.filter.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<T, F, Error> FilterOperator<T, F, Error> {
	pub fn new(filter: F) -> Self {
		Self {
			filter,
			_phantom_data: PhantomData,
		}
	}
}

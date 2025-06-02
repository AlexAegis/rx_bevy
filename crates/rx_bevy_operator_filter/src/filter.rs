use std::marker::PhantomData;

use rx_bevy_observable::{Observer, ObserverConnector};
use rx_bevy_operator::{ForwardObserver, Operator};

pub struct FilterOperator<T, Filter, Error> {
	pub filter: Filter,
	pub _phantom_data: PhantomData<(T, Error)>,
}

impl<T, Filter, Error> Operator for FilterOperator<T, Filter, Error>
where
	Filter: Clone + for<'a> Fn(&'a T) -> bool,
{
	type In = T;
	type Out = T;
	type InError = Error;
	type OutError = Error;

	type InternalSubscriber = Self;

	fn operator_subscribe<
		Destination: 'static + Observer<In = Self::Out, Error = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> ForwardObserver<Self::InternalSubscriber, Destination> {
		ForwardObserver::new(self.clone(), destination)
	}
}

impl<T, Filter, Error> ObserverConnector for FilterOperator<T, Filter, Error>
where
	Filter: for<'a> Fn(&'a T) -> bool,
{
	type In = T;
	type Out = T;
	type InError = Error;
	type OutError = Error;

	fn push_forward<Destination: Observer<In = T>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		if (self.filter)(&next) {
			destination.on_push(next);
		}
	}

	fn error_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		destination.on_error(error);
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

use std::marker::PhantomData;

use rx_bevy_observable::{ConnectorObserver, DynConnectorObserver, Observer};
use rx_bevy_operator::{ForwardObserver, Operator};

pub struct FilterOperator<T, Filter> {
	pub filter: Filter,
	pub _phantom_data_t: PhantomData<T>,
}

impl<T, Filter> Operator for FilterOperator<T, Filter>
where
	Filter: Clone + for<'a> Fn(&'a T) -> bool,
{
	type In = T;
	type Out = T;

	type InternalSubscriber = Self;

	fn operator_subscribe<Destination: 'static + Observer<In = Self::Out>>(
		&mut self,
		destination: Destination,
	) -> ForwardObserver<Self::InternalSubscriber, Destination> {
		ForwardObserver::new(self.clone(), destination)
	}
}

impl<T, Filter> ConnectorObserver for FilterOperator<T, Filter>
where
	Filter: for<'a> Fn(&'a T) -> bool,
{
	type In = T;
	type Out = T;

	// fn push_forward(&mut self, next: Self::In, destination: &mut dyn Observer<T>) {
	// 	if (self.filter)(&next) {
	// 		destination.on_push(next);
	// 	}
	// }

	fn push_forward<Destination: Observer<In = T>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		if (self.filter)(&next) {
			destination.on_push(next);
		}
	}
}

impl<T, F> Clone for FilterOperator<T, F>
where
	F: Clone,
{
	fn clone(&self) -> Self {
		Self {
			filter: self.filter.clone(),
			_phantom_data_t: PhantomData,
		}
	}
}

impl<T, F> FilterOperator<T, F> {
	pub fn new(filter: F) -> Self {
		Self {
			_phantom_data_t: PhantomData,
			filter,
		}
	}
}

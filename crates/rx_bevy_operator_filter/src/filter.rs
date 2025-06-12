use std::marker::PhantomData;

use rx_bevy_observable::{
	Forwarder, ObservableOutput, Observer, ObserverInput, Operator, Subscriber,
};

pub struct FilterOperator<In, Filter, Error> {
	pub filter: Filter,
	pub _phantom_data: PhantomData<(In, Error)>,
}

impl<In, Filter, Error> Operator for FilterOperator<In, Filter, Error>
where
	Filter: Clone + for<'a> Fn(&'a In) -> bool,
{
	type Fw = FilterForwarder<In, Filter, Error>;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Fw, Destination> {
		Subscriber::new(
			destination,
			FilterForwarder {
				filter: self.filter.clone(),
				_phantom_data: PhantomData,
			},
		)
	}
}

pub struct FilterForwarder<In, Filter, Error> {
	pub filter: Filter,
	pub _phantom_data: PhantomData<(In, Error)>,
}

impl<T, Filter, Error> ObservableOutput for FilterForwarder<T, Filter, Error>
where
	Filter: for<'a> Fn(&'a T) -> bool,
{
	type Out = T;
	type OutError = Error;
}

impl<In, Filter, Error> ObserverInput for FilterForwarder<In, Filter, Error>
where
	Filter: for<'a> Fn(&'a In) -> bool,
{
	type In = In;
	type InError = Error;
}

impl<T, Filter, Error> Forwarder for FilterForwarder<T, Filter, Error>
where
	Filter: for<'a> Fn(&'a T) -> bool,
{
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
	fn error_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
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

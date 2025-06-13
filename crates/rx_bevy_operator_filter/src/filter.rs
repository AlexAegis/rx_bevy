use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, ObservableOutput, Observer, ObserverInput, Operator};

pub struct FilterOperator<In, InError, Filter> {
	pub filter: Filter,
	pub _phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Filter> Operator for FilterOperator<In, InError, Filter>
where
	Filter: Clone + for<'a> Fn(&'a In) -> bool,
{
	type Fw = FilterForwarder<In, InError, Filter>;

	#[inline]
	fn create_instance(&self) -> Self::Fw {
		Self::Fw::new(self.filter.clone())
	}
}

pub struct FilterForwarder<In, InError, Filter> {
	pub filter: Filter,
	pub _phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Filter> FilterForwarder<In, InError, Filter> {
	pub fn new(filter: Filter) -> Self {
		Self {
			filter,
			_phantom_data: PhantomData,
		}
	}
}

impl<T, InError, Filter> ObservableOutput for FilterForwarder<T, InError, Filter>
where
	Filter: for<'a> Fn(&'a T) -> bool,
{
	type Out = T;
	type OutError = InError;
}

impl<In, InError, Filter> ObserverInput for FilterForwarder<In, InError, Filter>
where
	Filter: for<'a> Fn(&'a In) -> bool,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Filter> Forwarder for FilterForwarder<In, InError, Filter>
where
	Filter: for<'a> Fn(&'a In) -> bool,
{
	#[inline]
	fn next_forward<Destination: Observer<In = In>>(
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

impl<In, InError, Filter> Clone for FilterOperator<In, InError, Filter>
where
	Filter: Clone,
{
	fn clone(&self) -> Self {
		Self {
			filter: self.filter.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Filter> FilterOperator<In, InError, Filter> {
	pub fn new(filter: Filter) -> Self {
		Self {
			filter,
			_phantom_data: PhantomData,
		}
	}
}

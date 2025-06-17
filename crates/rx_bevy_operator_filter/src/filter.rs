use std::marker::PhantomData;

use rx_bevy_observable::{
	ClosableDestination, ObservableOutput, Observer, ObserverInput, Operation, Operator,
	Subscriber, Subscription,
};

pub struct FilterOperator<In, InError, Filter> {
	pub filter: Filter,
	pub _phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Filter> Operator for FilterOperator<In, InError, Filter>
where
	Filter: 'static + Clone + for<'a> Fn(&'a In) -> bool,
	In: 'static,
	InError: 'static,
{
	type Subscriber<D: Subscriber<In = Self::Out, InError = Self::OutError>> =
		FilterSubscriber<In, InError, Filter, D>;

	fn operator_subscribe<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		FilterSubscriber::new(destination, self.filter.clone())
	}
}

impl<In, InError, Filter> ObserverInput for FilterOperator<In, InError, Filter>
where
	Filter: for<'a> Fn(&'a In) -> bool,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Filter> ObservableOutput for FilterOperator<In, InError, Filter>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
{
	type Out = In;
	type OutError = InError;
}

pub struct FilterSubscriber<In, InError, Filter, Destination>
where
	Destination: Observer,
{
	destination: ClosableDestination<Destination>,
	filter: Filter,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Filter, Destination> FilterSubscriber<In, InError, Filter, Destination>
where
	Destination: Observer,
{
	pub fn new(destination: Destination, filter: Filter) -> Self {
		Self {
			destination: ClosableDestination::new(destination),
			filter,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Filter, Destination> ObserverInput
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Observer,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Filter, Destination> ObservableOutput
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Observer,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Filter, Destination> Observer
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if (self.filter)(&next) {
			self.destination.next(next);
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<In, InError, Filter, Destination> Operation
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Destination = Destination;
}

impl<In, InError, Filter, Destination> Subscription
	for FilterSubscriber<In, InError, Filter, Destination>
where
	In: 'static,
	InError: 'static,
	Filter: for<'a> Fn(&'a In) -> bool,
	Destination: Observer<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
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

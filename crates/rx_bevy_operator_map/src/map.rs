use std::marker::PhantomData;

use rx_bevy_observable::{DynObserverConnector, Observer, ObserverConnector};
use rx_bevy_operator::{ForwardObserver, Operator, OperatorCallback};

pub struct MapOperator<In, Out, F> {
	pub mapper: F,
	pub _phantom_data_in: PhantomData<In>,
	pub _phantom_data_out: PhantomData<Out>,
}

impl<In, Out, Mapper> Operator for MapOperator<In, Out, Mapper>
where
	Mapper: OperatorCallback<In, Out> + Clone,
{
	type In = In;
	type Out = Out;

	type InternalSubscriber = MapSubscriber<In, Out, Mapper>;

	fn operator_subscribe<Destination: 'static + Observer<In = Self::Out>>(
		&mut self,
		destination: Destination,
	) -> ForwardObserver<Self::InternalSubscriber, Destination> {
		ForwardObserver::new(
			MapSubscriber {
				_phantom_data_in: PhantomData,
				_phantom_data_out: PhantomData,
				mapper: self.mapper.clone(),
			},
			destination,
		)
	}
}

pub struct MapSubscriber<In, Out, F> {
	pub mapper: F,
	pub _phantom_data_in: PhantomData<In>,
	pub _phantom_data_out: PhantomData<Out>,
}

impl<In, Out, F> ObserverConnector for MapSubscriber<In, Out, F>
where
	F: Fn(In) -> Out,
{
	type In = In;
	type Out = Out;

	fn push_forward<Destination: Observer<In = Out>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		let mapped = (self.mapper)(next);
		destination.on_push(mapped);
	}
}

impl<In, Out, F> MapOperator<In, Out, F> {
	pub fn new(transform: F) -> Self {
		Self {
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
			mapper: transform,
		}
	}
}

impl<In, Out, F> Clone for MapOperator<In, Out, F>
where
	F: Clone,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
		}
	}
}

use std::marker::PhantomData;

use rx_bevy_observable::{ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::TakeSubscriber;

pub struct TakeOperator<In, InError> {
	pub count: usize,
	pub _phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> TakeOperator<In, InError> {
	pub fn new(count: usize) -> Self {
		Self {
			count,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Operator for TakeOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		TakeSubscriber<In, InError, Destination>;

	fn operator_subscribe<
		Destination: 'static
			+ Subscriber<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		TakeSubscriber::new(destination, self.count)
	}
}

impl<In, InError> ObserverInput for TakeOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> ObservableOutput for TakeOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError> Clone for TakeOperator<In, InError> {
	fn clone(&self) -> Self {
		Self {
			count: self.count,
			_phantom_data: PhantomData,
		}
	}
}

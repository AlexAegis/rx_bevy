use std::marker::PhantomData;

use rx_bevy_observable::{ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::SkipSubscriber;

/// The [SkipOperator] is used to skip the first `n` emissions of an observable,
/// after which it does nothing.
pub struct SkipOperator<In, InError> {
	pub count: usize,
	pub _phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> SkipOperator<In, InError> {
	pub fn new(count: usize) -> Self {
		Self {
			count,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Operator for SkipOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		SkipSubscriber<In, InError, Destination>;

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
		SkipSubscriber::new(destination, self.count)
	}
}

impl<In, InError> ObserverInput for SkipOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> ObservableOutput for SkipOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError> Clone for SkipOperator<In, InError> {
	fn clone(&self) -> Self {
		Self {
			count: self.count,
			_phantom_data: PhantomData,
		}
	}
}

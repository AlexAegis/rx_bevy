use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::EnumerateSubscriber;

/// The [EnumerateOperator] counts emissions, and downstream receives this
/// counter in a tuple with the emitted value as (T, usize)
pub struct EnumerateOperator<In, InError>
where
	InError: 'static,
{
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Default for EnumerateOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Operator for EnumerateOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		EnumerateSubscriber<In, InError, Destination>;

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
		EnumerateSubscriber::new(destination)
	}
}

impl<In, InError> ObserverInput for EnumerateOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> ObservableOutput for EnumerateOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Out = (In, usize);
	type OutError = InError;
}

impl<In, InError> Clone for EnumerateOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

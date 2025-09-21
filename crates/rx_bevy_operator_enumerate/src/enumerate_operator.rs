use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, SignalContext, Subscriber};

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
	type Subscriber<Destination>
		= EnumerateSubscriber<In, InError, Destination>
	where
		Destination: Subscriber<In = Self::Out, InError = Self::OutError>;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Self::Subscriber<Destination> as SignalContext>::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: Subscriber<In = Self::Out, InError = Self::OutError>,
	{
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

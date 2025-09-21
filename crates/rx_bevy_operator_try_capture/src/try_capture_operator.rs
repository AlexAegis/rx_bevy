use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, SignalContext, Subscriber};

use crate::TryCaptureSubscriber;

/// The [TryCaptureOperator] is used to pack incoming values and errors into a
/// Result. When used, upstream errors are guaranteed to not reach downstream.
pub struct TryCaptureOperator<In, InError>
where
	InError: 'static,
{
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Default for TryCaptureOperator<In, InError>
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

impl<In, InError> Operator for TryCaptureOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Subscriber<Destination>
		= TryCaptureSubscriber<In, InError, Destination>
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
		TryCaptureSubscriber::new(destination)
	}
}

impl<In, InError> ObserverInput for TryCaptureOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> ObservableOutput for TryCaptureOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Out = Result<In, InError>;
	type OutError = ();
}

impl<In, InError> Clone for TryCaptureOperator<In, InError>
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

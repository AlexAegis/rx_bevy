use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, Subscriber};

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
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		TryCaptureSubscriber<In, InError, Destination>;

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

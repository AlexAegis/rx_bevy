use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, SignalContext, Subscriber};

use crate::LiftOptionSubscriber;

pub struct LiftOptionOperator<In, InError> {
	pub _phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Default for LiftOptionOperator<In, InError> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Operator for LiftOptionOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Subscriber<Destination>
		= LiftOptionSubscriber<In, InError, Destination>
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
		LiftOptionSubscriber::new(destination)
	}
}

impl<In, InError> ObserverInput for LiftOptionOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = Option<In>;
	type InError = InError;
}

impl<In, InError> ObservableOutput for LiftOptionOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError> Clone for LiftOptionOperator<In, InError> {
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

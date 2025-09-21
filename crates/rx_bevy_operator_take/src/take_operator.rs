use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, SignalContext, Subscriber};

use crate::TakeSubscriber;

#[derive(Debug)]
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
	type Subscriber<Destination>
		= TakeSubscriber<In, InError, Destination>
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

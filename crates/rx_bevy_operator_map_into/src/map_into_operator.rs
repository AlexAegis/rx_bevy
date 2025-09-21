use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, SignalContext, Subscriber};

use crate::MapIntoSubscriber;

/// The [MapIntoOperator] calls `into()` to map incoming values to the expected
/// out value provided `From` is implemented on the downstream type.
/// When both `In` and `Out`, and `InError` and `OutError` types are the same,
/// it's equivalent to the `identity` operator and is a noop.
pub struct MapIntoOperator<In, InError, Out, OutError> {
	pub _phantom_data: PhantomData<(In, InError, Out, OutError)>,
}

impl<In, InError, Out, OutError> Default for MapIntoOperator<In, InError, Out, OutError> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Out, OutError> Operator for MapIntoOperator<In, InError, Out, OutError>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
{
	type Subscriber<Destination>
		= MapIntoSubscriber<In, InError, Out, OutError, Destination>
	where
		Destination: Subscriber<In = Self::Out, InError = Self::OutError>;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Self::Subscriber<Destination> as SignalContext>::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: Subscriber<In = Self::Out, InError = Self::OutError>,
	{
		MapIntoSubscriber::new(destination)
	}
}

impl<In, InError, Out, OutError> ObservableOutput for MapIntoOperator<In, InError, Out, OutError>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
{
	type Out = Out;
	type OutError = OutError;
}

impl<In, InError, Out, OutError> ObserverInput for MapIntoOperator<In, InError, Out, OutError>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Out, OutError> Clone for MapIntoOperator<In, InError, Out, OutError> {
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

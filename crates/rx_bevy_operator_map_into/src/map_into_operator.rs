use std::marker::PhantomData;

use rx_bevy_core::{DropContext, ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::MapIntoSubscriber;

/// The [MapIntoOperator] calls `into()` to map incoming values to the expected
/// out value provided `From` is implemented on the downstream type.
/// When both `In` and `Out`, and `InError` and `OutError` types are the same,
/// it's equivalent to the `identity` operator and is a noop.
pub struct MapIntoOperator<In, InError, Out, OutError, Context = ()> {
	pub _phantom_data: PhantomData<(In, InError, Out, OutError, Context)>,
}

impl<In, InError, Out, OutError, Context> Default
	for MapIntoOperator<In, InError, Out, OutError, Context>
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Out, OutError, Context> Operator
	for MapIntoOperator<In, InError, Out, OutError, Context>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
	Context: DropContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= MapIntoSubscriber<In, InError, Out, OutError, Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		MapIntoSubscriber::new(destination)
	}
}

impl<In, InError, Out, OutError, Context> ObservableOutput
	for MapIntoOperator<In, InError, Out, OutError, Context>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
{
	type Out = Out;
	type OutError = OutError;
}

impl<In, InError, Out, OutError, Context> ObserverInput
	for MapIntoOperator<In, InError, Out, OutError, Context>
where
	In: 'static + Into<Out>,
	InError: 'static + Into<OutError>,
	Out: 'static,
	OutError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Out, OutError, Context> Clone
	for MapIntoOperator<In, InError, Out, OutError, Context>
{
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

use std::marker::PhantomData;

use rx_bevy_core::{DropContext, ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::EnumerateSubscriber;

/// The [EnumerateOperator] counts emissions, and downstream receives this
/// counter in a tuple with the emitted value as (T, usize)
pub struct EnumerateOperator<In, InError, Context = ()>
where
	InError: 'static,
{
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> Default for EnumerateOperator<In, InError, Context>
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

impl<In, InError, Context> Operator for EnumerateOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= EnumerateSubscriber<In, InError, Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		EnumerateSubscriber::new(destination)
	}
}

impl<In, InError, Context> ObserverInput for EnumerateOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for EnumerateOperator<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	type Out = (In, usize);
	type OutError = InError;
}

impl<In, InError, Context> Clone for EnumerateOperator<In, InError, Context>
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

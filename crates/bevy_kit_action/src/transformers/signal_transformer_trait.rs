use std::fmt::Debug;

use crate::{Clock, ReflectBound, SerializeBound, Signal};

use super::SignalTransformContext;

pub trait SignalTransformer<C: Clock>:
	Default + Debug + Clone + ReflectBound + SerializeBound
{
	type InputSignal: Signal;
	type OutputSignal: Signal;

	/// Its result will be stored in a SocketConnectorTerminal
	fn transform(
		&mut self,
		signal: &Self::InputSignal,
		context: SignalTransformContext<'_, C, Self::InputSignal, Self::OutputSignal>,
	) -> Self::OutputSignal;
}

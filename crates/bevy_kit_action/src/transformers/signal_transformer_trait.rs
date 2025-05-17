use std::{any::TypeId, fmt::Debug};

use crate::{Clock, ReflectBound, SerializeBound, Signal};

use super::SignalTransformContext;

pub type InputOutputSignalKey = (TypeId, TypeId);

pub trait SignalTransformer: Default + Debug + Clone + ReflectBound + SerializeBound {
	type InputSignal: Signal;
	type OutputSignal: Signal;

	/// Its result will be stored in a SocketConnectorTerminal
	fn transform<C: Clock>(
		&mut self,
		signal: &Self::InputSignal,
		context: SignalTransformContext<'_, C, Self::InputSignal, Self::OutputSignal>,
	) -> Self::OutputSignal;

	fn signal_key() -> InputOutputSignalKey {
		(
			std::any::TypeId::of::<Self::InputSignal>(),
			std::any::TypeId::of::<Self::OutputSignal>(),
		)
	}
}

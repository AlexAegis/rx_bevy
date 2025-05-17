use std::{any::TypeId, fmt::Debug};

use crate::{Action, Clock, ReflectBound, SerializeBound, Signal};

use super::SignalTransformContext;

#[cfg(feature = "reflect")]
use bevy::reflect::Reflect;

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug))]
pub(crate) struct ActionKeyPair(TypeId, TypeId);

impl ActionKeyPair {
	pub(crate) fn from_actions<InputAction: Action, OutputAction: Action>() -> Self {
		Self(
			std::any::TypeId::of::<InputAction>(),
			std::any::TypeId::of::<OutputAction>(),
		)
	}
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug))]
pub(crate) struct SignalKeyPair(TypeId, TypeId);

impl SignalKeyPair {
	pub(crate) fn from_signals<InputSignal: Signal, OutputSignal: Signal>() -> Self {
		Self(
			std::any::TypeId::of::<InputSignal>(),
			std::any::TypeId::of::<OutputSignal>(),
		)
	}

	pub(crate) fn from_actions<InputAction: Action, OutputAction: Action>() -> Self {
		SignalKeyPair::from_signals::<InputAction::Signal, OutputAction::Signal>()
	}
}

pub trait SignalTransformer: Default + Debug + Clone + ReflectBound + SerializeBound {
	type InputSignal: Signal;
	type OutputSignal: Signal;

	/// Its result will be stored in a SocketConnectorTerminal
	fn transform<C: Clock>(
		&mut self,
		signal: &Self::InputSignal,
		context: SignalTransformContext<'_, C, Self::InputSignal>,
	) -> Self::OutputSignal;

	fn signal_key() -> SignalKeyPair {
		SignalKeyPair::from_signals::<Self::InputSignal, Self::OutputSignal>()
	}
}

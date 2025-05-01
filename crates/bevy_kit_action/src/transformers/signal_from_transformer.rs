use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Clock, Signal};

use super::SignalTransformer;

#[derive(Resource, Debug, Clone, Reflect)]
#[derive_where(Default)]
pub struct SignalFromTransformer<FromSignal: Signal, ToSignal: Signal + From<FromSignal>> {
	#[reflect(ignore)]
	_phantom_data_signal: PhantomData<FromSignal>,
	#[reflect(ignore)]
	_phantom_data_to_signal: PhantomData<ToSignal>,
}

impl<FromSignal: Signal, ToSignal: Signal + From<FromSignal>, C: Clock> SignalTransformer<C>
	for SignalFromTransformer<FromSignal, ToSignal>
{
	type InputSignal = FromSignal;
	type OutputSignal = ToSignal;

	fn transform(
		&mut self,
		signal: &Self::InputSignal,
		_context: super::SignalTransformContext<'_, C, Self::InputSignal, Self::OutputSignal>,
	) -> Self::OutputSignal {
		ToSignal::from(*signal)
	}
}

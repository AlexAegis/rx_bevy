use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Clock, Signal};

use super::SignalTransformer;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[derive_where(Default)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Default, Clone, Debug))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub struct SignalFromTransformer<FromSignal: Signal, ToSignal: Signal + From<FromSignal>> {
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<(FromSignal, ToSignal)>,
}

impl<FromSignal: Signal, ToSignal: Signal + From<FromSignal>> SignalTransformer
	for SignalFromTransformer<FromSignal, ToSignal>
{
	type InputSignal = FromSignal;
	type OutputSignal = ToSignal;

	fn transform<C: Clock>(
		&mut self,
		signal: &Self::InputSignal,
		_context: super::SignalTransformContext<'_, C, Self::InputSignal>,
	) -> Self::OutputSignal {
		ToSignal::from(*signal)
	}
}

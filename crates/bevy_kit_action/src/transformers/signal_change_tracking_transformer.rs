use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Clock, Signal};

use super::SignalTransformer;

// #[cfg(feature = "serialize")]
// use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[derive_where(Default)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Clone, Debug))]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(
// 	all(feature = "serialize", feature = "reflect"),
// 	reflect(Serialize, Deserialize)
// )]
pub struct ChangeTrackingTransformer<S: Signal> {
	#[reflect(ignore)]
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal + PartialEq, C: Clock> SignalTransformer<C> for ChangeTrackingTransformer<S> {
	type InputSignal = S;
	type OutputSignal = bool;

	fn transform(
		&mut self,
		signal: &Self::InputSignal,
		context: super::SignalTransformContext<'_, C, Self::InputSignal, Self::OutputSignal>,
	) -> Self::OutputSignal {
		signal == context.last_frame_input_signal
	}
}

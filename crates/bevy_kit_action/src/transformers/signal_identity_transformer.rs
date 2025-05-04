use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Clock, Signal};

use super::{SignalTransformContext, SignalTransformer};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[derive_where(Default)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Clone, Debug))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub struct IdentitySignalTransformer<S: Signal> {
	#[cfg_attr(feature = "serialize", serde(bound(deserialize = "S: Signal")))]
	buffer: S,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal, C: Clock> SignalTransformer<C> for IdentitySignalTransformer<S> {
	type InputSignal = S;
	type OutputSignal = Self::InputSignal;

	fn transform(
		&mut self,
		signal: &Self::InputSignal,
		_context: SignalTransformContext<'_, C, Self::InputSignal, Self::OutputSignal>,
	) -> Self::OutputSignal {
		*signal
	}
}

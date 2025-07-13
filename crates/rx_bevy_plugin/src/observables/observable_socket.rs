use bevy_ecs::component::Component;
use rx_bevy_observable::ObservableOutput;

#[cfg(feature = "debug")]
use std::fmt::Debug;
use std::marker::PhantomData;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::ObservableSignalBound;

#[derive(Component, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ObservableSocket<Out, OutError> {
	_phantom_pain: PhantomData<(Out, OutError)>,
}

impl<Out, OutError> ObservableSocket<Out, OutError>
where
	Out: ObservableSignalBound,
	OutError: ObservableSignalBound,
{
	pub fn new() -> Self {
		Self {
			_phantom_pain: PhantomData,
		}
	}
}

impl<Out, OutError> ObservableOutput for ObservableSocket<Out, OutError>
where
	Out: ObservableSignalBound,
	OutError: ObservableSignalBound,
{
	type Out = Out;
	type OutError = OutError;
}

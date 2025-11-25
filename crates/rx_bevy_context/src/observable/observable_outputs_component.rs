use std::marker::PhantomData;

use bevy_ecs::component::Component;
use derive_where::derive_where;
use rx_core_traits::SignalBound;

#[derive_where(Default)]
#[derive(Component, Debug)]
pub struct ObservableOutputs<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	_phantom_data: PhantomData<fn() -> (Out, OutError)>,
}

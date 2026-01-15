use bevy_ecs::component::Component;
use derive_where::derive_where;
use rx_core_common::{PhantomInvariant, Signal};

#[derive_where(Default)]
#[derive(Component, Debug)]
pub struct ObservableOutputs<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	_phantom_data: PhantomInvariant<(Out, OutError)>,
}

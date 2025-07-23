use rx_bevy_common_bounds::{DebugBound, ReflectBound};

pub trait ObservableSignalBound: 'static + Send + Sync + Clone + ReflectBound + DebugBound {}

impl<T> ObservableSignalBound for T where
	T: 'static + Send + Sync + Clone + ReflectBound + DebugBound
{
}

use rx_bevy_common_bounds::{DebugBound, ReflectBound};

pub trait SignalBound: 'static + Send + Sync + Clone + ReflectBound + DebugBound {}

impl<T> SignalBound for T where T: 'static + Send + Sync + Clone + ReflectBound + DebugBound {}

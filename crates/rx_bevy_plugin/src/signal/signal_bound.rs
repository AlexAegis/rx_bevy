use rx_bevy_common_bounds::DebugBound;

pub trait SignalBound: 'static + Send + Sync + Clone + DebugBound {}

impl<T> SignalBound for T where T: 'static + Send + Sync + Clone + DebugBound {}

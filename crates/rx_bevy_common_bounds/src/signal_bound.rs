use crate::DebugBound;

pub trait SignalBound: 'static + Send + Sync + Clone + DebugBound {}

impl<T> SignalBound for T where T: 'static + Send + Sync + Clone + DebugBound {}

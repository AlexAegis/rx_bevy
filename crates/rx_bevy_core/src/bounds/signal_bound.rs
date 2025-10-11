pub trait SignalBound: 'static + Send + Sync {}

impl<T> SignalBound for T where T: 'static + Send + Sync {}

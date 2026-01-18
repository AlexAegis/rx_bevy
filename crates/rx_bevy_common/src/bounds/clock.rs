pub trait Clock: Default + Copy + Clone + Send + Sync + 'static {}

impl<T> Clock for T where T: Default + Copy + Clone + Send + Sync + 'static {}

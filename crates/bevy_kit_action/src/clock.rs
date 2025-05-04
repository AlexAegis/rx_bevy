use crate::ReflectBound;

pub trait Clock: Default + Copy + Clone + Send + Sync + ReflectBound + 'static {}

impl<T> Clock for T where T: Default + Copy + Clone + Send + Sync + ReflectBound + 'static {}

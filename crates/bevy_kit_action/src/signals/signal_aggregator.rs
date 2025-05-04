use std::fmt::Debug;

use crate::{ReflectBound, SerializeBound};

use super::Signal;

pub trait SignalAggregator<S: Signal>:
	Default + Debug + Send + Sync + 'static + ReflectBound + SerializeBound
{
	fn combine(&self, signals: impl Iterator<Item = S>) -> S;
}

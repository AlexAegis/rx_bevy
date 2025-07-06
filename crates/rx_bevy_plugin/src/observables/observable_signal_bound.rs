use crate::{DebugBound, ReflectBound};

pub trait ObservableSignalBound: Send + Sync + Clone + ReflectBound + DebugBound {}

impl<T> ObservableSignalBound for T where T: Send + Sync + Clone + ReflectBound + DebugBound {}

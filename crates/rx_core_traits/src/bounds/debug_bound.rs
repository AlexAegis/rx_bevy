#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(not(feature = "debug"))]
pub trait DebugBound {}
#[cfg(not(feature = "debug"))]
impl<T> DebugBound for T {}

#[cfg(feature = "debug")]
pub trait DebugBound: Debug {}
#[cfg(feature = "debug")]
impl<T: Debug> DebugBound for T {}

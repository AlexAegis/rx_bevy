#[cfg(feature = "reflect")]
use bevy_reflect::FromReflect;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflectable;

#[cfg(not(feature = "reflect"))]
pub trait ReflectBound {}
#[cfg(not(feature = "reflect"))]
impl<T> ReflectBound for T {}
#[cfg(feature = "reflect")]
pub trait ReflectBound: Reflectable + FromReflect {}
#[cfg(feature = "reflect")]
impl<T: Reflectable + FromReflect> ReflectBound for T {}

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "serialize"))]
pub trait SerializeBound {}
#[cfg(not(feature = "serialize"))]
impl<T> SerializeBound for T {}
#[cfg(feature = "serialize")]
pub trait SerializeBound: Serialize + for<'de> Deserialize<'de> {}
#[cfg(feature = "serialize")]
impl<T: Serialize + for<'de> Deserialize<'de>> SerializeBound for T {}

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

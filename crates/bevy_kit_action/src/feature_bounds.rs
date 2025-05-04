use bevy::reflect::FromReflect;
#[cfg(feature = "reflect")]
use bevy::reflect::Reflectable;
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

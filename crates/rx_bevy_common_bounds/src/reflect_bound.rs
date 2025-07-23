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

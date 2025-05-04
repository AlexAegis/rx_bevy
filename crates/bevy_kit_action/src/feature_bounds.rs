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
//
// #[cfg(feature = "serialize")]
// use serde::{Serialize, de::DeserializeOwned};
// #[cfg(not(feature = "serialize"))]
// pub trait SerializeBound {}
// #[cfg(not(feature = "serialize"))]
// impl<T> SerializeBound for T {}
// #[cfg(feature = "serialize")]
// pub trait SerializeBound: Serialize + DeserializeOwned {}
// #[cfg(feature = "serialize")]
// impl<T: Serialize + DeserializeOwned> SerializeBound for T {}

// pub trait ReflectBound {}
// impl<T> ReflectBound for T {}

pub trait SerializeBound {}
impl<T> SerializeBound for T {}

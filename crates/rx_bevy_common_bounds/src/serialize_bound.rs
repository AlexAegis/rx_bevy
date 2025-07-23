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

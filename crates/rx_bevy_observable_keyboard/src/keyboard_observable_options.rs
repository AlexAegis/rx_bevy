#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone, Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct KeyboardObservableOptions {}

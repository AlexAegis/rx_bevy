use std::{fmt::Debug, hash::Hash};

/// - GetTypeRegistration + Typed: Only if the "reflect" feature is enabled

#[cfg(feature = "reflect")]
use bevy::reflect::{GetTypeRegistration, Typed};
#[cfg(not(feature = "reflect"))]
pub trait ActionKeyBound: Send + Sync + 'static {}
#[cfg(not(feature = "reflect"))]
impl<T: Send + Sync + 'static> ActionKeyBound for T {}
/// GetTypeRegistration + Typed implies Send + Sync and 'static anyway
#[cfg(feature = "reflect")]
pub trait ActionKeyBound: GetTypeRegistration + Typed {}
#[cfg(feature = "reflect")]
impl<T: GetTypeRegistration + Typed> ActionKeyBound for T {}

/// The reason Actions are (usually) just unit structs, is that they are used as identifiers,
/// if you want to store data along with an action, use the associated ActionData type
///
/// Required supertraits and their reasons:
/// - Debug: For debugging
/// - Eq + Hash: Used as a key in HashMaps
/// - Copy: Mapping involves cloning the keys to use in two iterators
/// - Send + Sync
/// - 'static
///
pub trait ActionKey: Copy + Eq + Hash + Debug + ActionKeyBound {
	type ActionData: Default + Debug + ActionKeyBound;
}

/// TODO: Unsure if needed
pub(crate) enum ActionDimension {
	Digital,
	Analog2d,
	Analog3d,
}

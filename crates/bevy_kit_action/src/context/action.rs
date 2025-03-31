use crate::{InputSocket, OutputSocket, Signal};
use std::{fmt::Debug, hash::Hash};

#[cfg(feature = "reflect")]
use bevy::reflect::{GetTypeRegistration, Typed};

#[cfg(not(feature = "reflect"))]
pub trait ActionBound: Send + Sync + 'static {}
#[cfg(not(feature = "reflect"))]
impl<T: Send + Sync + 'static> ActionBound for T {}
/// GetTypeRegistration + Typed implies Send + Sync and 'static anyway
#[cfg(feature = "reflect")]
pub trait ActionBound: GetTypeRegistration + Typed {}
#[cfg(feature = "reflect")]
impl<T: GetTypeRegistration + Typed> ActionBound for T {}

/// The reason Actions are (usually) just unit structs, is that they are used as identifiers,
/// if you want to store data along with an action, use the associated Signal type
///
/// Required supertraits and their reasons:
/// - Debug: For debugging
/// - Eq + Hash: Used as a key in HashMaps
/// - Copy: Mapping involves cloning the keys to use in two iterators
/// - Send + Sync
/// - GetTypeRegistration + Typed: Only if the "reflect" feature is enabled
/// - 'static
///
pub trait Action: Copy + Eq + Hash + Debug + ActionBound {
	type Signal: Signal;
	// TODO Does it actually need a separate input and output socket if there's only one signal?
	/// How to get activated
	type InputSocket: InputSocket;
	/// What is outputted
	type OutputSocket: OutputSocket;
}

/*
pub trait Action: Copy + Eq + Hash + Debug + ActionBound {
	type Signal: Default + Debug + ActionBound;
}

*/

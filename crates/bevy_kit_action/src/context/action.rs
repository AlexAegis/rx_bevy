use crate::{ReflectBound, SerializeBound, Signal};
use std::{fmt::Debug, hash::Hash};

// TODO: Maybe rename Actions to Channel, or Wire or something
/// The reason Actions are (usually) just unit structs, is that they are used as identifiers,
/// if you want to store data along with an action, use the associated Signal type
///
/// Required supertraits and their reasons:
/// - Debug: For debugging
/// - Eq + Hash: Used as a key in HashMaps
/// - Clone + Copy: Mapping involves cloning the keys to use in two iterators
/// - Send + Sync
/// - GetTypeRegistration + Typed: Only if the "reflect" feature is enabled
/// - 'static
pub trait Action:
	Clone + Copy + Eq + Hash + Debug + Send + Sync + 'static + ReflectBound + SerializeBound
{
	// What is passed into a compatible socket
	type Signal: Signal;
}

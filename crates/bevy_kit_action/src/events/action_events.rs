use bevy::prelude::*;

use crate::ActionKey;

#[derive(Event, Debug)]
pub struct ActionStart<A: ActionKey> {
	pub action: A,
}

#[derive(Event, Debug)]
pub struct ActionOnGoing<A: ActionKey> {
	pub action: A,
}

#[derive(Event, Debug)]
pub struct ActionEnd<A: ActionKey> {
	pub action: A,
}

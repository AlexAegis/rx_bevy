use bevy::prelude::*;

use crate::Action;

#[derive(Event, Debug)]
pub struct ActionStart<A: Action> {
	pub action: A,
}

#[derive(Event, Debug)]
pub struct ActionOnGoing<A: Action> {
	pub action: A,
}

#[derive(Event, Debug)]
pub struct ActionEnd<A: Action> {
	pub action: A,
}

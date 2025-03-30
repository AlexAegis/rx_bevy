use std::marker::PhantomData;

use crate::ActionKey;
use bevy::{prelude::*, time::Stopwatch, utils::HashMap};
use derive_where::derive_where;

/// Where Actions arrive from
#[derive(Component, Clone, Debug, Reflect)]
#[derive_where(Default)]
pub struct ActionSource<A: ActionKey> {
	_p: PhantomData<A>,
	pub sources: Vec<Entity>,
}

use std::marker::PhantomData;

use crate::Action;
use bevy::{prelude::*, time::Stopwatch, utils::HashMap};
use derive_where::derive_where;

/// Where Actions arrive from, removing or modifying this is an easy way to
/// disable this action from being triggered from other sources, but it could
/// still get manually triggered.
#[derive(Component, Clone, Debug, Reflect)]
#[derive_where(Default)]
pub struct ActionSource<A: Action> {
	_p: PhantomData<A>,
	pub sources: Vec<Entity>,
}

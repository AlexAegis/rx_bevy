use crate::{ObservableComponent, ObservableOnSubscribeContext, setup_observable_hook};
use crate::{ObservableOnInsertContext, RxNext};
use bevy::ecs::component::{Mutable, StorageType};
use bevy::prelude::*;

use derive_where::derive_where;
use rx_bevy::prelude::*;

// TODO: This could be combined into a single derive for ObservableComponent
#[derive(Clone)]
// TODO(bevy-0.17): Derive Component && #[component(on_insert = setup_observable::<Self>(on_subscribe))]
#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
pub struct IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + IntoIterator + Send + Sync,
	Iterator::Item: 'static + Send + Sync,
{
	iterator: Iterator,
}

/// TODO: Abstract this away, this is what makes an ObservableComponent Subscribable
impl<Iterator> Component for IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + IntoIterator + Send + Sync,
	Iterator::Item: 'static + Send + Sync,
{
	const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn on_insert() -> Option<bevy::ecs::component::ComponentHook> {
		Some(setup_observable_hook::<Self>)
	}
}

impl<Iterator> IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + IntoIterator + Send + Sync,
	Iterator::Item: 'static + Send + Sync,
{
	pub fn new(iterator: Iterator) -> Self {
		Self { iterator }
	}
}

impl<Iterator> ObservableComponent for IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + IntoIterator + Send + Sync,
	Iterator::Item: 'static + Send + Sync,
{
	const CAN_SELF_SUBSCRIBE: bool = true;

	fn on_insert(&mut self, _commands: &mut Commands, _context: ObservableOnInsertContext) {}

	fn on_subscribe(&mut self, commands: &mut Commands, context: ObservableOnSubscribeContext) {
		for item in self.iterator.clone().into_iter() {
			commands.trigger_targets(RxNext(item), context.subscriber_entity);
		}
	}
}

impl<Iterator> ObservableOutput for IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + IntoIterator + Send + Sync,
	Iterator::Item: 'static + Send + Sync,
{
	type Out = Iterator::Item;
	type OutError = ();
}

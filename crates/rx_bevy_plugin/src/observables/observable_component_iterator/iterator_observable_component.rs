use crate::{CommandSubscriber, IteratorSubscription, ObservableOnInsertContext};
use crate::{
	ObservableComponent, SignalBound, observable_on_insert_hook, observable_on_remove_hook,
};

use bevy_ecs::component::{Component, ComponentHooks, Mutable, StorageType};

use rx_bevy_common_bounds::{DebugBound, ReflectBound};
use rx_bevy_observable::{ObservableOutput, Observer};

#[cfg(feature = "debug")]
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone)]
#[cfg_attr(
	feature = "debug",
	derive_where(Debug),
	derive_where(skip_inner(Debug))
)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IteratorObservableComponent<Iterator, const EMIT_ON_TICK: bool>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	iterator: Iterator,
}

/// TODO: Abstract this away, this is what makes an ObservableComponent Subscribable
impl<Iterator, const EMIT_ON_TICK: bool> Component
	for IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone + ReflectBound,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	const STORAGE_TYPE: StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut ComponentHooks) {
		hooks.on_insert(observable_on_insert_hook::<Self>);
		hooks.on_remove(observable_on_remove_hook::<<Self as ObservableComponent>::Subscription>);
	}
}

impl<Iterator, const EMIT_ON_TICK: bool> IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	pub fn new(iterator: Iterator) -> Self {
		Self { iterator }
	}
}

impl<Iterator, const EMIT_ON_TICK: bool> ObservableComponent
	for IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone + ReflectBound,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	const CAN_SELF_SUBSCRIBE: bool = true;

	type Subscription = IteratorSubscription<Iterator, EMIT_ON_TICK>;

	fn on_insert(&mut self, _context: ObservableOnInsertContext) {}

	fn on_subscribe(
		&mut self,
		mut subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) -> Self::Subscription {
		if !EMIT_ON_TICK {
			for item in self.iterator.clone().into_iter() {
				subscriber.next(item);
			}
			subscriber.complete();
		}

		IteratorSubscription::new(self.iterator.clone())
	}
}

impl<Iterator, const EMIT_ON_TICK: bool> ObservableOutput
	for IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	type Out = Iterator::Item;
	type OutError = ();
}

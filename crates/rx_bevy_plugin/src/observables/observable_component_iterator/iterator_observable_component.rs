use crate::{CommandSubscriber, IteratorSubscription, OnInsertSubHook};
use crate::{
	ObservableComponent, SignalBound, observable_on_insert_hook, observable_on_remove_hook,
};

use bevy_ecs::component::Component;

use rx_bevy_common_bounds::{DebugBound, ReflectBound};
use rx_bevy_observable::{ObservableOutput, Observer};

#[cfg(feature = "debug")]
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Component, Clone)]
#[component(on_insert = observable_on_insert_hook::<Self>, on_remove = observable_on_remove_hook::<<Self as ObservableComponent>::Subscription>)]
#[cfg_attr(
	feature = "debug",
	derive_where(Debug),
	derive_where(skip_inner(Debug))
)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IteratorObservableComponent<Iterator, const EMIT_ON_TICK: bool>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone + ReflectBound,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	iterator: Iterator,
}

impl<Iterator, const EMIT_ON_TICK: bool> IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone + ReflectBound,
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

impl<Iterator, const EMIT_ON_TICK: bool> OnInsertSubHook
	for IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone + ReflectBound,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	fn on_insert(&mut self, _context: crate::ObservableOnInsertContext) {}
}

impl<Iterator, const EMIT_ON_TICK: bool> ObservableOutput
	for IteratorObservableComponent<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator + Send + Sync + Clone + ReflectBound,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	type Out = Iterator::Item;
	type OutError = ();
}

use rx_bevy_observable::ObservableOutput;

use crate::{
	DebugBound, ObservableSignalBound, RxNext, RxTick, ScheduledSubscription, SubscriptionContext,
};

use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg_attr(feature = "debug", derive_where(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IteratorSubscription<Iterator, const EMIT_ON_TICK: bool>
where
	Iterator: IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	iterator: Iterator::IntoIter,
}

impl<Iterator, const EMIT_ON_TICK: bool> IteratorSubscription<Iterator, EMIT_ON_TICK>
where
	Iterator: IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	pub fn new(iterator: Iterator) -> Self {
		Self {
			iterator: iterator.into_iter(),
		}
	}
}

impl<Iterator, const EMIT_ON_TICK: bool> ObservableOutput
	for IteratorSubscription<Iterator, EMIT_ON_TICK>
where
	Iterator: IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator, const EMIT_ON_TICK: bool> ScheduledSubscription
	for IteratorSubscription<Iterator, EMIT_ON_TICK>
where
	Iterator: IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: 'static + ObservableSignalBound,
{
	const SCHEDULED: bool = EMIT_ON_TICK;

	fn on_tick(&mut self, _event: &RxTick, context: SubscriptionContext) {
		if let Some(next) = self.iterator.next() {
			context
				.commands
				.trigger_targets(RxNext(next), context.subscriber_entity);
		}
		// TODO: Else complete
	}

	fn unsubscribe(&mut self, _context: SubscriptionContext) {}
}

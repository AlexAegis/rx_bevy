use rx_bevy_observable::{ObservableOutput, Observer, SubscriptionLike};

use crate::{CommandSubscriber, DebugBound, ObservableSignalBound, RxTick, ScheduledSubscription};

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

	fn on_tick(
		&mut self,
		_event: &RxTick,
		mut destination: CommandSubscriber<Self::Out, Self::OutError>,
	) {
		if let Some(next) = self.iterator.next() {
			destination.next(next);
		} else {
			destination.complete();
		}
	}

	fn unsubscribe(&mut self, mut destination: CommandSubscriber<Self::Out, Self::OutError>) {
		destination.unsubscribe();
	}
}

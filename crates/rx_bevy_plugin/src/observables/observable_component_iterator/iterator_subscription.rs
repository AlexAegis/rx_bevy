use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, Observer, SubscriptionLike, Tick};

use crate::{CommandSubscriber, RxSubscription, SignalBound};

#[cfg(feature = "debug")]
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg_attr(feature = "debug", derive_where(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IteratorSubscription<Iterator, const EMIT_ON_TICK: bool>
where
	Iterator: IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	iterator: Iterator::IntoIter,
}

impl<Iterator, const EMIT_ON_TICK: bool> IteratorSubscription<Iterator, EMIT_ON_TICK>
where
	Iterator: IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
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
	Iterator::Item: SignalBound,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator, const EMIT_ON_TICK: bool> RxSubscription
	for IteratorSubscription<Iterator, EMIT_ON_TICK>
where
	Iterator: IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	const SCHEDULED: bool = EMIT_ON_TICK;

	fn on_tick(
		&mut self,
		_tick: Tick,
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

use bevy_ecs::observer::Trigger;
use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, Observer};

use crate::{CommandSubscriber, RxContextSub, RxDestination, RxSubscription, RxTick, SignalBound};

#[cfg(feature = "debug")]
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg_attr(feature = "debug", derive_where(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IteratorSubscription<Iterator, const EMIT_ON_TICK: bool>
where
	Iterator: 'static + IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	iterator: Iterator::IntoIter,
}

impl<Iterator, const EMIT_ON_TICK: bool> IteratorSubscription<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator,
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
	Iterator: 'static + IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator, const EMIT_ON_TICK: bool> RxSubscription
	for IteratorSubscription<Iterator, EMIT_ON_TICK>
where
	Iterator: 'static + IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	const SCHEDULED: bool = EMIT_ON_TICK;

	fn register_channel_handlers<'a, 'w, 's>(
		&mut self,
		hooks: &mut crate::SubscriptionChannelHandlerRegistrationContext<'a, 'w, 's, Self>,
	) {
		if EMIT_ON_TICK {
			hooks.register_tick_handler(iterator_subscriber_on_tick::<Iterator>);
		}
	}

	fn unsubscribe(&mut self, mut destination: CommandSubscriber<Self::Out, Self::OutError>) {
		destination.unsubscribe();
	}
}

fn iterator_subscriber_on_tick<Iterator>(
	trigger: Trigger<RxTick>,
	mut context: RxContextSub<IteratorSubscription<Iterator, true>>,
	mut destination: RxDestination<IteratorSubscription<Iterator, true>>,
) where
	Iterator: 'static + IntoIterator,
	Iterator::IntoIter: 'static + Send + Sync + DebugBound,
	Iterator::Item: SignalBound,
{
	let mut subscription = context.get_subscription(trigger.target());
	let mut subscriber = destination.get_destination(trigger.target());

	if let Some(next) = subscription.iterator.next() {
		subscriber.next(next);
	} else {
		subscriber.complete();
	}
}

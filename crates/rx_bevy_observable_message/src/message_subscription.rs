use bevy_ecs::event::{Event, EventCursor, Events};
use rx_bevy_context::{BevySubscriptionContext, BevySubscriptionContextProvider};
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Subscriber, SubscriptionContext, SubscriptionData, SubscriptionLike, Teardown,
	TeardownCollection, Tick, Tickable,
};

#[derive(RxSubscription)]
#[rx_context(BevySubscriptionContextProvider)]
pub struct MessageSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
	Destination::In: Event + Clone, // TODO(bevy-0.17): use the message trait
{
	destination: Destination,
	message_cursor: EventCursor<Destination::In>,
	teardown: SubscriptionData<BevySubscriptionContextProvider>,
}

impl<Destination> MessageSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
	Destination::In: Event + Clone,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			message_cursor: EventCursor::default(),
			teardown: SubscriptionData::default(),
		}
	}
}

impl<Destination> SubscriptionLike for MessageSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
	Destination::In: Event + Clone,
{
	#[inline]
	#[track_caller]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.destination.unsubscribe(context);
			self.teardown.unsubscribe(context);
		}
	}
}

impl<Destination> TeardownCollection for MessageSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
	Destination::In: Event + Clone,
{
	#[track_caller]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.teardown.add_teardown(teardown, context);
		} else {
			teardown.execute(context);
		}
	}
}

impl<Destination> Tickable for MessageSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
	Destination::In: Event + Clone,
{
	#[track_caller]
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		let events = context.deferred_world.resource::<Events<Destination::In>>();

		let read_events = self
			.message_cursor
			.read(events)
			.cloned()
			.collect::<Vec<_>>();

		for event in read_events {
			self.destination.next(event, context);
		}

		self.destination.tick(tick, context);
	}
}

impl<Destination> Drop for MessageSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
	Destination::In: Event + Clone,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context =
				BevySubscriptionContextProvider::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}

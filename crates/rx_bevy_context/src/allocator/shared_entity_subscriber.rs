use core::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_core_traits::{
	Observer, ObserverInput, ObserverUpgradesToSelf, Subscriber, SubscriberNotification,
	SubscriptionClosedFlag, SubscriptionLike, Teardown, TeardownCollection, Tick, Tickable,
	WithSubscriptionContext, allocator::SharedDestination,
};

use crate::{BevySubscriptionContext, BevySubscriptionContextProvider, SubscriberComponent};

/// An easily clonable subscriber that does not own its destination, only points
/// to it. It is not a component and is only used internally in other subscribers.
#[deprecated = "maybe giving these an entity is a bad idea, it is with the switch, has to be tried with subjects too"]
pub struct SharedEntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	destination_entity: Entity,
	/// TODO: The shared heap subs use this field, but we don't know the type of destination. so maybe add a new component SubscriptionClosed(bool)
	closed_flag: SubscriptionClosedFlag,
	_phantom_data: PhantomData<Destination>,
}

impl<Destination> SharedEntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	pub(crate) fn new(destination_entity: Entity) -> Self {
		Self {
			destination_entity,
			closed_flag: false.into(),
			_phantom_data: PhantomData,
		}
	}

	#[inline]
	pub fn get_destination_entity(&self) -> Entity {
		self.destination_entity
	}

	pub fn destination_exists(&self, context: &mut BevySubscriptionContext) -> bool {
		context
			.deferred_world
			.entities()
			.contains(self.destination_entity)
	}
}

impl<Destination> Clone for SharedEntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn clone(&self) -> Self {
		Self {
			destination_entity: self.destination_entity,
			closed_flag: self.closed_flag.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Destination> SharedDestination<Destination> for SharedEntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn clone_with_context(&self, _context: &mut BevySubscriptionContext<'_, '_>) -> Self {
		Self {
			destination_entity: self.destination_entity,
			closed_flag: self.closed_flag.clone(),
			_phantom_data: PhantomData,
		}
	}

	fn access_with_context<F>(&mut self, accessor: F, context: &mut BevySubscriptionContext<'_, '_>)
	where
		F: Fn(&Destination, &mut BevySubscriptionContext<'_, '_>),
	{
		// SAFETY: The allocator ensures [SharedEntitySubscriber] only points to entities
		// with a []
		let stolen_destination = context
			.get_expected_component_mut::<SubscriberComponent<Destination>>(self.destination_entity)
			.steal_destination();

		accessor(&stolen_destination, context);

		context
			.get_expected_component_mut::<SubscriberComponent<Destination>>(self.destination_entity)
			.return_stolen_destination(stolen_destination);
	}

	fn access_with_context_mut<F>(
		&mut self,
		mut accessor: F,
		context: &mut BevySubscriptionContext<'_, '_>,
	) where
		F: FnMut(&mut Destination, &mut BevySubscriptionContext<'_, '_>),
	{
		let mut stolen_destination = context
			.get_expected_component_mut::<SubscriberComponent<Destination>>(self.destination_entity)
			.steal_destination();

		accessor(&mut stolen_destination, context);

		context
			.get_expected_component_mut::<SubscriberComponent<Destination>>(self.destination_entity)
			.return_stolen_destination(stolen_destination);
	}
}

impl<Destination> ObserverInput for SharedEntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithSubscriptionContext for SharedEntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	type Context = BevySubscriptionContextProvider;
}

impl<Destination> ObserverUpgradesToSelf for SharedEntitySubscriber<Destination> where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>
{
}

impl<Destination> Observer for SharedEntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn next(&mut self, next: Self::In, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<
					Destination::In,
					Destination::InError,
					Self::Context,
				>::Next(next),
			);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<
					Destination::In,
					Destination::InError,
					Self::Context,
				>::Error(error),
			);
		}
	}

	fn complete(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<
					Destination::In,
					Destination::InError,
					Self::Context,
				>::Complete,
			);
		}
	}
}

impl<Destination> Tickable for SharedEntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		// Tick must not be stopped even if it's closed, in case a
		// downstream subscription is expecting it
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Tick(
				tick,
			),
		);
	}
}

impl<Destination> SubscriptionLike for SharedEntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();
			context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Unsubscribe,
		);
			context
				.deferred_world
				.commands()
				.entity(self.destination_entity)
				.despawn();
		}
	}
}

impl<Destination> TeardownCollection for SharedEntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut BevySubscriptionContext<'_, '_>,
	) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Add(
					Some(teardown),
				),
			);
		} else {
			teardown.execute(context);
		}
	}
}

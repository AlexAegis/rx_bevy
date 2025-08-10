use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, system::Commands};

use rx_bevy_observable::{ObserverInput, SubscriptionLike};
use smallvec::SmallVec;

use crate::{ObserverSignalPush, RxSignal, SignalBound};

#[cfg(feature = "debug")]
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg_attr(feature = "debug", derive_where(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct CommandSubscriber<'a, 'w, 's, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[cfg_attr(feature = "debug", derive_where(skip))]
	commands: &'a mut Commands<'w, 's>,
	/// "Destination" entity
	destination_entity: Entity,

	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	subscription_entity: Entity,

	closed: bool,

	_phantom_data: PhantomData<(In, InError)>,
}

impl<'a, 'w, 's, In, InError> CommandSubscriber<'a, 'w, 's, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	pub fn unsubscribe(&mut self) {
		if !self.closed {
			self.closed = true;
			self.commands.entity(self.subscription_entity).despawn();
		}
	}

	pub fn downgrade(self) -> SubscriberContext<In, InError> {
		SubscriberContext {
			destination_entity: self.destination_entity,
			subscription_entity: self.subscription_entity,
			closed: self.closed,
			buffer: SmallVec::default(),
			_phantom_data: PhantomData,
		}
	}

	#[inline]
	pub fn commands(&mut self) -> &mut Commands<'w, 's> {
		self.commands
	}

	#[inline]
	pub fn get_destination_entity(&self) -> Entity {
		self.destination_entity
	}

	#[inline]
	pub fn get_subscription_entity(&self) -> Entity {
		self.subscription_entity
	}
}

impl<'a, 'w, 's, In, InError> ObserverInput for CommandSubscriber<'a, 'w, 's, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<'a, 'w, 's, In, InError> rx_bevy_observable::Observer
	for CommandSubscriber<'a, 'w, 's, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn next(&mut self, next: Self::In) {
		if !self.closed {
			self.commands
				.trigger_targets(RxSignal::<In, InError>::Next(next), self.destination_entity);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.closed {
			self.commands.trigger_targets(
				RxSignal::<In, InError>::Error(error),
				self.destination_entity,
			);
		}
	}

	fn complete(&mut self) {
		if !self.closed {
			self.commands
				.trigger_targets(RxSignal::<In, InError>::Complete, self.destination_entity);
			self.unsubscribe();
		}
	}

	fn tick(&mut self, tick: rx_bevy_observable::Tick) {
		if !self.closed {
			self.commands.trigger_targets(tick, self.destination_entity);
		}
	}
}

/// This intermediate struct is used to avoid mixing up the three entities
pub struct EntityContext {
	/// The "destination" entity, where signals are sent.
	pub destination_entity: Entity,
	/// Despawning this stops the subscription, and is equivalent of an
	/// unsubscribe.
	pub subscription_entity: Entity,
}

#[cfg_attr(feature = "debug", derive_where(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriberContext<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	/// The "destination" entity, where signals are sent.
	destination_entity: Entity,
	/// Despawning this stops the subscription, and is equivalent of an
	/// unsubscribe.
	subscription_entity: Entity,
	closed: bool,

	buffer: SmallVec<[RxSignal<In, InError>; 2]>,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> SubscriberContext<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	pub fn new(entity_context: EntityContext) -> Self {
		Self {
			destination_entity: entity_context.destination_entity,
			subscription_entity: entity_context.subscription_entity,
			closed: false,
			buffer: SmallVec::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> SubscriberContext<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	pub fn upgrade<'a, 'w, 's>(
		self,
		commands: &'a mut Commands<'w, 's>,
	) -> CommandSubscriber<'a, 'w, 's, In, InError>
	where
		In: SignalBound,
		InError: SignalBound,
	{
		CommandSubscriber::<'a, 'w, 's, In, InError> {
			commands,
			destination_entity: self.destination_entity,
			subscription_entity: self.subscription_entity,
			closed: self.closed,
			_phantom_data: PhantomData,
		}
	}

	/// Drains the buffer into a [CommandSubscriber]
	pub(crate) fn forward_buffer<'a, 'w, 's>(
		&mut self,
		command_subscriber: &mut CommandSubscriber<'a, 'w, 's, In, InError>,
	) {
		for signal in self.buffer.drain(..) {
			command_subscriber.push(signal);
		}
	}
}

impl<In, InError> ObserverInput for SubscriberContext<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> rx_bevy_observable::Observer for SubscriberContext<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn next(&mut self, next: Self::In) {
		self.buffer.push(RxSignal::Next(next));
	}

	fn error(&mut self, error: Self::InError) {
		self.buffer.push(RxSignal::Error(error));
	}

	fn complete(&mut self) {
		self.buffer.push(RxSignal::Complete);
	}

	fn tick(&mut self, _tick: rx_bevy_observable::Tick) {
		// TODO: Should this be buffered?
		// unreachable!("ticked the context")
		// Don't need to do anything with a tick here
	}
}

impl<In, InError> SubscriptionLike for SubscriberContext<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		if !self.closed {
			self.closed = true;
		}
	}

	fn add(&mut self, _subscription: Box<dyn SubscriptionLike>) {
		// TODO: Maybe buffer this too? Realistically this would only be an entity
		unreachable!("Can't add subscriptionLikes to tear down")
	}
}

use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, system::Commands};
use bevy_log::debug;
use derive_where::derive_where;
use rx_bevy_observable::{ObserverInput, SubscriptionLike};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{DebugBound, ObservableSignalBound, RxSignal};

#[cfg_attr(feature = "debug", derive_where(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct CommandSubscriber<'a, 'w, 's, In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	#[derive_where(skip)]
	commands: &'a mut Commands<'w, 's>,
	/// "This" entity
	source_entity: Entity,
	/// "Destination" entity
	destination_entity: Entity,

	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	subscription_entity: Entity,

	closed: bool,

	// #[derive_where(skip)]
	// #[reflect(ignore)]
	// teardown: InnerSubscription,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<'a, 'w, 's, In, InError> CommandSubscriber<'a, 'w, 's, In, InError>
where
	In: 'static + ObservableSignalBound,
	InError: 'static + ObservableSignalBound,
{
	pub fn downgrade(self) -> SubscriberContext<In, InError> {
		SubscriberContext {
			source_entity: self.source_entity,
			destination_entity: self.destination_entity,
			subscription_entity: self.subscription_entity,
			closed: self.closed,
			// teardown: self.teardown,
			_phantom_data: PhantomData,
		}
	}
}

impl<'a, 'w, 's, In, InError> ObserverInput for CommandSubscriber<'a, 'w, 's, In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	type In = In;
	type InError = InError;
}

impl<'a, 'w, 's, In, InError> rx_bevy_observable::Observer
	for CommandSubscriber<'a, 'w, 's, In, InError>
where
	In: 'static + Send + Sync + DebugBound,
	InError: 'static + Send + Sync + DebugBound,
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
}

impl<'a, 'w, 's, In, InError> SubscriptionLike for CommandSubscriber<'a, 'w, 's, In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		if !self.closed {
			self.closed = true;
			// self.teardown.unsubscribe();
			debug!("CommandSubscriber unsubscribe");
			self.commands.entity(self.subscription_entity).despawn();
		}
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		// self.teardown.add(Teardown::Sub(subscription));
	}
}

/// This intermediate struct is used to avoid mixing up the three entities
pub struct EntityContext {
	/// "This" entity, usually an observable
	pub source_entity: Entity,
	/// "Destination" entity
	pub destination_entity: Entity,
	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	pub subscription_entity: Entity,
}

#[cfg_attr(feature = "debug", derive_where(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriberContext<In, InError>
where
	In: 'static + Send + Sync + ObservableSignalBound,
	InError: 'static + Send + Sync + ObservableSignalBound,
{
	/// "This" entity
	source_entity: Entity,
	/// "Destination" entity
	destination_entity: Entity,
	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	subscription_entity: Entity,

	closed: bool,

	// #[derive_where(skip)]
	// #[reflect(ignore)]
	// teardown: InnerSubscription,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> SubscriberContext<In, InError>
where
	In: 'static + Send + Sync + ObservableSignalBound,
	InError: 'static + Send + Sync + ObservableSignalBound,
{
	pub fn new(entity_context: EntityContext) -> Self {
		Self {
			source_entity: entity_context.source_entity,
			destination_entity: entity_context.destination_entity,
			subscription_entity: entity_context.subscription_entity,
			closed: false,
			// teardown: InnerSubscription::new_empty(),
			_phantom_data: PhantomData,
		}
	}

	#[inline]
	pub fn get_observable_entity(&self) -> Entity {
		self.source_entity
	}
}

impl<In, InError> SubscriberContext<In, InError>
where
	In: 'static + Send + Sync + ObservableSignalBound,
	InError: 'static + Send + Sync + ObservableSignalBound,
{
	pub fn upgrade<'a, 'w, 's>(
		self,
		commands: &'a mut Commands<'w, 's>,
	) -> CommandSubscriber<'a, 'w, 's, In, InError>
	where
		In: 'static + Send + Sync,
		InError: 'static + Send + Sync,
	{
		CommandSubscriber::<'a, 'w, 's, In, InError> {
			commands,
			source_entity: self.source_entity,
			destination_entity: self.destination_entity,
			subscription_entity: self.subscription_entity,
			closed: self.closed,
			// teardown: InnerSubscription::new_empty(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ObserverInput for SubscriberContext<In, InError>
where
	In: 'static + Send + Sync + ObservableSignalBound,
	InError: 'static + Send + Sync + ObservableSignalBound,
{
	type In = In;
	type InError = InError;
}

// TODO: Maybe this impl should just be removed and accept that subscriber context is not a subscriber
impl<In, InError> rx_bevy_observable::Observer for SubscriberContext<In, InError>
where
	In: 'static + Send + Sync + ObservableSignalBound,
	InError: 'static + Send + Sync + ObservableSignalBound,
{
	fn next(&mut self, next: Self::In) {
		// TODO: Maybe collect in a buffer then drain on upgrade? Or panic if not supposed to receive anything un-upgraded
		println!("SubscriptionEntityContext next {:?}", next);
	}

	fn error(&mut self, error: Self::InError) {
		println!("SubscriptionEntityContext error {:?}", error);
	}

	fn complete(&mut self) {
		println!("SubscriptionEntityContext complete");
	}
}

impl<In, InError> rx_bevy_observable::SubscriptionLike for SubscriberContext<In, InError>
where
	In: 'static + Send + Sync + ObservableSignalBound,
	InError: 'static + Send + Sync + ObservableSignalBound,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		if !self.closed {
			self.closed = true;
			// self.teardown.unsubscribe();
		}
	}

	fn add(&mut self, subscription: &'static mut dyn rx_bevy_observable::SubscriptionLike) {
		//	self.teardown.add(Teardown::Sub(subscription));
	}
}

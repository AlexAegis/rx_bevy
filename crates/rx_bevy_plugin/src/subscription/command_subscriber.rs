use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, system::Commands};
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;
use rx_bevy_observable::{InnerSubscription, ObserverInput, SubscriptionLike, Teardown};

use crate::RxNext;

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
	observable_entity: Entity,
	/// "Destination" entity
	subscriber_entity: Entity,

	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	subscription_entity: Entity,

	closed: bool,

	#[derive_where(skip)]
	#[reflect(ignore)]
	teardown: InnerSubscription,

	_phantom_data: PhantomData<(In, InError)>,
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
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	fn next(&mut self, next: Self::In) {
		if !self.closed {
			self.commands
				.trigger_targets(RxNext(next), self.subscriber_entity);
		}
	}

	fn error(&mut self, _error: Self::InError) {
		if !self.closed {
			//todo!("impl")
		}
	}

	fn complete(&mut self) {
		if !self.closed {
			self.unsubscribe();
			//	todo!("impl sending complete event");
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
			self.teardown.unsubscribe();
			self.commands.entity(self.subscription_entity).despawn();
		}
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.teardown.add(Teardown::Sub(subscription));
	}
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionEntityContext {
	/// "This" entity
	pub observable_entity: Entity,
	/// "Destination" entity
	pub subscriber_entity: Entity,
	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	pub subscription_entity: Entity,
}

impl SubscriptionEntityContext {
	pub fn upgrade<'a, 'w, 's, In, InError>(
		self,
		commands: &'a mut Commands<'w, 's>,
	) -> CommandSubscriber<'a, 'w, 's, In, InError>
	where
		In: 'static + Send + Sync,
		InError: 'static + Send + Sync,
	{
		CommandSubscriber::<'a, 'w, 's, In, InError> {
			commands,
			observable_entity: self.observable_entity,
			subscriber_entity: self.subscriber_entity,
			subscription_entity: self.subscription_entity,
			closed: false,
			teardown: InnerSubscription::new_empty(),
			_phantom_data: PhantomData,
		}
	}
}

use std::marker::PhantomData;

use bevy_ecs::{
	bundle::Bundle,
	entity::Entity,
	hierarchy::ChildOf,
	name::Name,
	observer::Observer,
	system::{Commands, IntoObserverSystem},
};
use short_type_name::short_type_name;

use crate::{
	RxChannelComplete, RxChannelError, RxChannelNext, RxChannelTick, RxChannelUnsubscribe,
	RxComplete, RxError, RxNext, RxSubscriber, RxSubscription, RxTick, RxUnsubscribe, SignalBound,
	SubscriptionChannelHandlerOf,
};

use derive_where::derive_where;

#[cfg_attr(feature = "debug", derive_where(Debug))]
pub struct SubscriptionChannelHandlerRegistrationContext<'a, 'w, 's, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	subscription: Entity,
	#[cfg_attr(feature = "debug", derive_where(skip))]
	commands: &'a mut Commands<'w, 's>,
	_phantom_data: PhantomData<Sub>,
}

impl<'a, 'w, 's, Sub> SubscriptionChannelHandlerRegistrationContext<'a, 'w, 's, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn new(subscription: Entity, commands: &'a mut Commands<'w, 's>) -> Self {
		Self {
			subscription,
			commands,
			_phantom_data: PhantomData,
		}
	}

	pub fn register_tick_handler<B, M>(
		&mut self,
		system: impl IntoObserverSystem<RxTick, B, M> + 'static,
	) where
		B: Bundle,
	{
		let subscriber_entity = self.subscription;
		self.commands.spawn((
			SubscriptionChannelHandlerOf::<RxChannelTick, Sub>::new(subscriber_entity),
			Name::new(format!("Tick Handler - {}", short_type_name::<Sub>())),
			ChildOf(subscriber_entity),
			Observer::new(system).with_entity(subscriber_entity),
		));
	}

	pub fn register_unsubscribe_handler<B, M>(
		&mut self,
		system: impl IntoObserverSystem<RxUnsubscribe, B, M> + 'static,
	) where
		B: Bundle,
	{
		let subscriber_entity = self.subscription;
		self.commands.spawn((
			SubscriptionChannelHandlerOf::<RxChannelUnsubscribe, Sub>::new(subscriber_entity),
			Name::new(format!(
				"Unsubscribe Handler - {}",
				short_type_name::<Sub>()
			)),
			ChildOf(subscriber_entity),
			Observer::new(system).with_entity(subscriber_entity),
		));
	}
}

#[cfg_attr(feature = "debug", derive_where(Debug))]
pub struct SubscriberChannelHandlerRegistrationContext<'a, 'w, 's, Sub>
where
	Sub: RxSubscriber,
	Sub::In: SignalBound,
	Sub::InError: SignalBound,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	subscription: Entity,
	#[cfg_attr(feature = "debug", derive_where(skip))]
	commands: &'a mut Commands<'w, 's>,
	_phantom_data: PhantomData<Sub>,
}

impl<'a, 'w, 's, Sub> SubscriberChannelHandlerRegistrationContext<'a, 'w, 's, Sub>
where
	Sub: RxSubscriber,
	Sub::In: SignalBound,
	Sub::InError: SignalBound,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn new(subscription: Entity, commands: &'a mut Commands<'w, 's>) -> Self {
		Self {
			subscription,
			commands,
			_phantom_data: PhantomData,
		}
	}

	pub fn register_next_handler<B, M>(
		&mut self,
		system: impl IntoObserverSystem<RxNext<Sub::In>, B, M> + 'static,
	) where
		B: Bundle,
	{
		let subscriber_entity = self.subscription;
		self.commands.spawn((
			SubscriptionChannelHandlerOf::<RxChannelNext, Sub>::new(subscriber_entity),
			Name::new(format!("Next Handler - {}", short_type_name::<Sub>())),
			ChildOf(subscriber_entity),
			Observer::new(system).with_entity(subscriber_entity),
		));
	}

	pub fn register_error_handler<B, M>(
		&mut self,
		system: impl IntoObserverSystem<RxError<Sub::InError>, B, M> + 'static,
	) where
		B: Bundle,
	{
		let subscriber_entity = self.subscription;
		self.commands.spawn((
			SubscriptionChannelHandlerOf::<RxChannelError, Sub>::new(subscriber_entity),
			Name::new(format!("Error Handler - {}", short_type_name::<Sub>())),
			ChildOf(subscriber_entity),
			Observer::new(system).with_entity(subscriber_entity),
		));
	}

	pub fn register_complete_handler<B, M>(
		&mut self,
		system: impl IntoObserverSystem<RxComplete, B, M> + 'static,
	) where
		B: Bundle,
	{
		let subscriber_entity = self.subscription;
		self.commands.spawn((
			SubscriptionChannelHandlerOf::<RxChannelComplete, Sub>::new(subscriber_entity),
			Name::new(format!("Complete Handler - {}", short_type_name::<Sub>())),
			ChildOf(subscriber_entity),
			Observer::new(system).with_entity(subscriber_entity),
		));
	}
}

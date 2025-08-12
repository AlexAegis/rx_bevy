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
	RxChannel, RxChannelAdd, RxChannelComplete, RxChannelError, RxChannelNext, RxChannelTick,
	RxChannelUnsubscribe, RxSubscriber, RxSubscription, SignalBound, SubscriptionChannelHandlerOf,
};

use derive_where::derive_where;

pub trait ChannelHandlerRegistrationContext<'a, 'w, 's, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	fn get_commands<'c>(&'c mut self) -> &'c mut Commands<'w, 's>;

	fn get_subscription_entity(&self) -> Entity;

	fn register_channel_handler<C, B, M>(
		&mut self,
		#[allow(unused, reason = "It's for inference")] channel: C,
		system: impl IntoObserverSystem<C::Event<Sub>, B, M> + 'static,
	) where
		C: RxChannel,
		B: Bundle,
	{
		let subscription_entity = self.get_subscription_entity();
		self.get_commands().spawn((
			SubscriptionChannelHandlerOf::<C, Sub>::new(subscription_entity),
			Name::new(format!("{:?} Handler", short_type_name::<C>())),
			ChildOf(subscription_entity),
			Observer::new(system),
		));
	}
}

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
		system: impl IntoObserverSystem<<RxChannelTick as RxChannel>::Event<Sub>, B, M> + 'static,
	) where
		B: Bundle,
	{
		self.register_channel_handler(RxChannelTick, system);
	}

	pub fn register_add_handler<B, M>(
		&mut self,
		system: impl IntoObserverSystem<<RxChannelAdd as RxChannel>::Event<Sub>, B, M> + 'static,
	) where
		B: Bundle,
	{
		self.register_channel_handler(RxChannelAdd, system);
	}

	pub fn register_unsubscribe_handler<B, M>(
		&mut self,
		system: impl IntoObserverSystem<<RxChannelUnsubscribe as RxChannel>::Event<Sub>, B, M> + 'static,
	) where
		B: Bundle,
	{
		self.register_channel_handler(RxChannelUnsubscribe, system);
	}
}

impl<'a, 'w, 's, Sub> ChannelHandlerRegistrationContext<'a, 'w, 's, Sub>
	for SubscriptionChannelHandlerRegistrationContext<'a, 'w, 's, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	fn get_commands<'c>(&'c mut self) -> &'c mut Commands<'w, 's> {
		self.commands
	}

	fn get_subscription_entity(&self) -> Entity {
		self.subscription
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

impl<'a, 'w, 's, Sub> ChannelHandlerRegistrationContext<'a, 'w, 's, Sub>
	for SubscriberChannelHandlerRegistrationContext<'a, 'w, 's, Sub>
where
	Sub: RxSubscriber,
	Sub::In: SignalBound,
	Sub::InError: SignalBound,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	fn get_commands<'c>(&'c mut self) -> &'c mut Commands<'w, 's> {
		self.commands
	}

	fn get_subscription_entity(&self) -> Entity {
		self.subscription
	}
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
		system: impl IntoObserverSystem<<RxChannelNext as RxChannel>::Event<Sub>, B, M> + 'static,
	) where
		B: Bundle,
	{
		self.register_channel_handler(RxChannelNext, system);
	}

	pub fn register_error_handler<B, M>(
		&mut self,
		system: impl IntoObserverSystem<<RxChannelError as RxChannel>::Event<Sub>, B, M> + 'static,
	) where
		B: Bundle,
	{
		self.register_channel_handler(RxChannelError, system);
	}

	pub fn register_complete_handler<B, M>(
		&mut self,
		system: impl IntoObserverSystem<<RxChannelComplete as RxChannel>::Event<Sub>, B, M> + 'static,
	) where
		B: Bundle,
	{
		self.register_channel_handler(RxChannelComplete, system);
	}
}

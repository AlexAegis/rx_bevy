use std::marker::PhantomData;

use bevy_ecs::{
	bundle::Bundle,
	entity::Entity,
	hierarchy::ChildOf,
	name::Name,
	observer::Observer,
	system::{Commands, IntoObserverSystem, SystemParam},
};
use short_type_name::short_type_name;

use crate::{RxChannel, RxSubscription, SignalBound, SubscriptionChannelHandlerOf};

use derive_where::derive_where;

#[cfg_attr(feature = "debug", derive_where(Debug))]
pub struct SubscriptionHookRegistrationContext<'a, 'w, 's, Sub>
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

impl<'a, 'w, 's, Sub> SubscriptionHookRegistrationContext<'a, 'w, 's, Sub>
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

	pub fn register_hook<C, B, M>(
		&mut self,
		#[allow(unused)] channel: C,
		system: impl IntoObserverSystem<C::Event<Sub>, B, M> + 'static,
	) where
		C: RxChannel,
		B: Bundle,
	{
		self.commands.spawn((
			SubscriptionChannelHandlerOf::<C, Sub>::new(self.subscription),
			Name::new(format!("{:?} Handler", short_type_name::<C>())),
			ChildOf(self.subscription),
			Observer::new(system),
		));
	}
}

/// TODO: Create a systemparam that could be used for system based signal handling with access to next/error/complete/unsubscribe? methods
#[derive(SystemParam)]

pub struct SubscriberSystemParam<'w, 's> {
	commands: Commands<'w, 's>,
}

impl<'w, 's> SubscriberSystemParam<'w, 's> {}

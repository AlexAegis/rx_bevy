use std::marker::PhantomData;

use bevy_ecs::{component::Component, entity::Entity};
use rx_bevy_common_bounds::SignalBound;

use crate::{RxChannel, RxSubscription};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Component)]
#[relationship(relationship_target=SubscriptionChannelHandlerRef::<Channel, Sub>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(type_path = false))]
pub struct SubscriptionChannelHandlerOf<Channel, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
	Channel: RxChannel,
{
	#[relationship]
	subscription: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<(Channel, Sub)>,
}

impl<Channel, Sub> SubscriptionChannelHandlerOf<Channel, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
	Channel: RxChannel,
{
	pub fn new(destination: Entity) -> Self {
		Self {
			subscription: destination,
			_phantom_data: PhantomData,
		}
	}
}

/// This semantically is a relationship but that imposes too many restrictions,
/// and subscriptions are managed uniquely anyways.
#[derive(Component)]
#[relationship_target(relationship=SubscriptionChannelHandlerOf::<Channel, Sub>, linked_spawn)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(type_path = false))]
pub struct SubscriptionChannelHandlerRef<Channel, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
	Channel: RxChannel,
{
	#[relationship]
	handler: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<(Channel, Sub)>,
}

impl<Channel, Sub> SubscriptionChannelHandlerRef<Channel, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
	Channel: RxChannel,
{
}

#[cfg(feature = "reflect")]
impl<Channel, Sub> bevy_reflect::TypePath for SubscriptionChannelHandlerOf<Channel, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
	Channel: RxChannel,
{
	fn crate_name() -> Option<&'static str> {
		Some("rx_bevy_plugin")
	}

	fn module_path() -> Option<&'static str> {
		Some("rx_bevy_plugin")
	}

	fn short_type_path() -> &'static str {
		"SubscriptionChannelHandlerOf"
	}

	fn type_ident() -> Option<&'static str> {
		Some("SubscriptionChannelHandlerOf")
	}
	fn type_path() -> &'static str {
		"rx_bevy_plugin::SubscriptionChannelHandlerOf"
	}
}

#[cfg(feature = "reflect")]
impl<Channel, Sub> bevy_reflect::TypePath for SubscriptionChannelHandlerRef<Channel, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
	Channel: RxChannel,
{
	fn crate_name() -> Option<&'static str> {
		Some("rx_bevy_plugin")
	}

	fn module_path() -> Option<&'static str> {
		Some("rx_bevy_plugin")
	}

	fn short_type_path() -> &'static str {
		"SubscriptionChannelHandlerRef"
	}

	fn type_ident() -> Option<&'static str> {
		Some("SubscriptionChannelHandlerRef")
	}
	fn type_path() -> &'static str {
		"rx_bevy_plugin::SubscriptionChannelHandlerRef"
	}
}

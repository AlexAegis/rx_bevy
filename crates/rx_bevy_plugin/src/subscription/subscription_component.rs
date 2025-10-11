use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
	component::{Component, HookContext},
	world::DeferredWorld,
};
use rx_bevy_core::{ObservableOutput, SignalBound};

use crate::RxSubscription;

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// The heart of a subscription, the actual instance created from either
/// an [ObservableComponent] or an [OperatorComponent].
/// This is not a relation, other components handle the relations of a
/// subscription entity, such as:
/// - [SubscriptionSignalSources]
///
/// While it could be part of the
#[derive(Component, Deref, DerefMut)]
#[component(on_remove=unsubscribe_subscription_on_remove::<Sub>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Subscription<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	#[deref]
	pub(crate) subscription: Sub,
}

impl<Sub> Subscription<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn new(subscription: Sub) -> Self {
		Self { subscription }
	}
}

fn unsubscribe_subscription_on_remove<Sub>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	let subscription_entity = hook_context.entity;

	let (mut entities, _commands) = deferred_world.entities_and_commands();

	let mut subscription_entity = entities.get_mut(subscription_entity).ok();
	let _subscription = subscription_entity
		.as_mut()
		.and_then(|e| e.get_mut::<Subscription<Sub>>())
		.expect("the component should be available");
	// TODO: Once it looks like this
	// subscription.unsubscribe(ChannelContext {
	// 	commands: &mut commands,
	// });
}

impl<Sub> ObservableOutput for Subscription<Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	type Out = Sub::Out;
	type OutError = Sub::OutError;
}

use bevy_ecs::component::Component;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// A simple marker component to easily identify all subscription entities,
/// only used for debugging.
#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionMarker;

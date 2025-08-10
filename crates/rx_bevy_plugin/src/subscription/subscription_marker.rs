use bevy_ecs::component::Component;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// A simple marker component to easily identify all subscription entities, but
/// not subscriber entities.
#[derive(Component)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionMarker;

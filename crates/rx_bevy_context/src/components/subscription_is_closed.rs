use bevy_ecs::component::Component;

/// A simple component to track the closedness of a [SubscriptionLike] without
/// needing to know the type of that.
/// Once marked as closed, it can't be un-closed, just like subscriptions and
/// subscribers.
///
/// Since subscriptions (as opposed to observables) each have their own entity,
/// this component does not have to be generic to ensure it refers to exactly
/// one subscription.
///
/// The component subscribers/subscriptions that use this must ensure that this
/// is a required component of it, and
#[derive(Component)]
pub struct SubscriptionIsClosed {
	closed: bool,
}

impl Default for SubscriptionIsClosed {
	fn default() -> Self {
		Self { closed: false }
	}
}

impl SubscriptionIsClosed {
	#[inline]
	pub fn is_closed(&self) -> bool {
		self.closed
	}

	#[inline]
	pub fn close(&mut self) {
		self.closed = true;
	}
}

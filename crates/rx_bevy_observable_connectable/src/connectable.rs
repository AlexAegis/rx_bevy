use rx_bevy_core::{Observable, SubscriptionLike, WithSubscriptionContext};

use crate::ConnectionHandle;

pub trait Connectable: Observable {
	// TODO: This does not need to be tickable
	type ConnectionSubscription: SubscriptionLike;

	fn connect(
		&mut self,
		context: &mut <Self::ConnectionSubscription as WithSubscriptionContext>::Context,
	) -> ConnectionHandle<Self::ConnectionSubscription>;
}

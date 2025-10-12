use rx_bevy_core::{Observable, SubscriptionLike, WithSubscriptionContext};

use crate::ConnectionHandle;

pub trait Connectable: Observable {
	type ConnectionSubscription: SubscriptionLike + Send + Sync;

	fn connect(
		&mut self,
		context: &mut <Self::ConnectionSubscription as WithSubscriptionContext>::Context,
	) -> ConnectionHandle<Self::ConnectionSubscription>;
}

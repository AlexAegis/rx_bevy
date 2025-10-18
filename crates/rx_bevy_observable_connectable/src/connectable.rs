use rx_bevy_core::{
	Observable, SubscriptionLike, context::WithSubscriptionContext, prelude::SubscriptionContext,
};

use crate::ConnectionHandle;

pub trait Connectable: Observable {
	type ConnectionSubscription: SubscriptionLike + Send + Sync;

	fn connect(
		&mut self,
		context: &mut <<Self::ConnectionSubscription as WithSubscriptionContext>::Context as SubscriptionContext>::Item<'_, '_>,
	) -> ConnectionHandle<Self::ConnectionSubscription>;
}

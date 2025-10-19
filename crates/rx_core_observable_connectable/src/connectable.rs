use rx_core_traits::{Observable, SubscriptionContext, SubscriptionLike, WithSubscriptionContext};

use crate::observable::ConnectionHandle;

pub trait Connectable: Observable {
	type ConnectionSubscription: SubscriptionLike + Send + Sync;

	fn connect(
		&mut self,
		context: &mut <<Self::ConnectionSubscription as WithSubscriptionContext>::Context as SubscriptionContext>::Item<'_, '_>,
	) -> ConnectionHandle<Self::ConnectionSubscription>;
}

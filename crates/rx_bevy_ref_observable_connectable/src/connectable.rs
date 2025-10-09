use rx_bevy_core::{Observable, SubscriptionLike, WithContext};

pub trait Connectable: Observable {
	type ConnectionSubscription: SubscriptionLike;

	fn connect(
		&mut self,
		context: &mut <Self::ConnectionSubscription as WithContext>::Context,
	) -> Self::ConnectionSubscription;
}

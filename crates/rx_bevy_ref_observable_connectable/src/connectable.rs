use rx_bevy_core::{Observable, SignalContext, SubscriptionLike};

pub trait Connectable: Observable {
	type ConnectionSubscription: SubscriptionLike;

	fn connect(
		&mut self,
		context: &mut <Self::ConnectionSubscription as SignalContext>::Context,
	) -> Self::ConnectionSubscription;
}

use rx_bevy_core::{Observable, SignalContext, SubscriptionLike};

pub trait Connectable: Observable {
	type ConnectionSubscription: SubscriptionLike;

	fn connect<'c>(
		&mut self,
		context: &mut <Self::ConnectionSubscription as SignalContext>::Context<'c>,
	) -> Self::ConnectionSubscription;
}

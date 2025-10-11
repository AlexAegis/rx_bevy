use rx_bevy_core::{Observable, SubscriptionHandle, TickableSubscription, WithSubscriptionContext};

pub trait Connectable: Observable {
	// TODO: This does not need to be tickable
	type ConnectionSubscription: TickableSubscription;

	fn connect(
		&mut self,
		context: &mut <Self::ConnectionSubscription as WithSubscriptionContext>::Context,
	) -> SubscriptionHandle<Self::ConnectionSubscription>;
}

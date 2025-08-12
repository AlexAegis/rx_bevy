use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, ObserverInput};

use crate::{
	CommandSubscriber, SignalBound, SubscriberChannelHandlerRegistrationContext,
	SubscriptionChannelHandlerRegistrationContext,
};

// TODO: CONTINUE
// TODO: This may need an add method for other subscriptions to tear down unsubscribe, or not, and have that work with other components
/// This trait is the bevy equivalent of a SubscriptionLike and an Observer
pub trait RxSubscription: 'static + ObservableOutput + DebugBound + Sized
where
	Self: Send + Sync,
	Self::Out: SignalBound,
	Self::OutError: SignalBound,
{
	/// When set to false, the subscription will not be ticked at all.
	const SCHEDULED: bool = true;

	fn register_channel_handlers<'a, 'w, 's>(
		&mut self,
		handlers: &mut SubscriptionChannelHandlerRegistrationContext<'a, 'w, 's, Self>,
	);

	/// Happens when either the [Subscription] or its relation from [Subscriptions] is removed
	///
	/// > Note that when this runs, this [ScheduledSubscription] instance is already removed
	/// > from the [SubscriptionComponent], not that you would ever try that, since `self` is that.
	fn unsubscribe(&mut self, subscriber: CommandSubscriber<Self::Out, Self::OutError>);
}

// TODO: Maybe it's okay to have these together and always have an on_signal impl for non transformer observables too, maybe it would enable per-subscription pipes
/// While in pure rust, a subscriber is something that's an observer and a
/// subscription at the same time, here in terms of ECS, this means an entity
/// that has both components.
pub trait RxSubscriber: ObserverInput + RxSubscription
where
	Self: Send + Sync,
	Self::In: SignalBound,
	Self::InError: SignalBound,
	Self::Out: SignalBound,
	Self::OutError: SignalBound,
{
	fn register_channel_handlers<'a, 'w, 's>(
		&mut self,
		handlers: &mut SubscriberChannelHandlerRegistrationContext<'a, 'w, 's, Self>,
	);
}

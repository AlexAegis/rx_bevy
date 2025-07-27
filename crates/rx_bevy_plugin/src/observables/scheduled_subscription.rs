use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, Tick};

use crate::CommandSubscriber;

pub trait ScheduledSubscription: ObservableOutput + DebugBound
where
	Self: Send + Sync,
	Self::Out: Send + Sync,
	Self::OutError: Send + Sync,
{
	/// When set to false, the subscription will not be ticked at all.
	const SCHEDULED: bool = true;

	fn on_tick(&mut self, tick: Tick, subscriber: CommandSubscriber<Self::Out, Self::OutError>);

	/// Happens when either the [Subscription] or its relation from [Subscriptions] is removed
	///
	/// > Note that when this runs, this [ScheduledSubscription] instance is already removed
	/// > from the [SubscriptionComponent], not that you would ever try that, since `self` is that.
	fn unsubscribe(&mut self, subscriber: CommandSubscriber<Self::Out, Self::OutError>);
}

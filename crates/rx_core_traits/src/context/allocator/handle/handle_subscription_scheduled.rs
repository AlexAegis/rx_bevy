use crate::SubscriptionScheduled;

use super::{UnscheduledSubscriptionHandle, WeakSubscriptionHandle};

/// # ScheduledSubscriptionHandle
///
/// The main handle for subscriptions that need scheduling. Only one can exist
/// for a single subscription and is the owner of that subscription.
///
/// These types do not implement [Clone], instead, they have their own `clone`
/// method that returns an [UnscheduledSubscriptionHandle], ensuring only one
/// reference exists for the subscription that can be scheduled.
///
/// By calling `downgrade` on such handle, one can acquire a clonable,
/// non-owning "weak" handle that can be used to unsubscribe the subscription.
pub trait ScheduledSubscriptionHandle: SubscriptionScheduled + Send + Sync {
	type UnscheduledHandle: UnscheduledSubscriptionHandle<Context = Self::Context>;
	type WeakHandle: WeakSubscriptionHandle<Context = Self::Context>;

	fn downgrade(&mut self) -> Self::WeakHandle;

	/// To ensure only one handle is scheduled, this "fake" clone method returns
	/// an [UnscheduledHandle][ScheduledSubscriptionHandle::UnscheduledHandle].
	fn clone(&self) -> Self::UnscheduledHandle;
}

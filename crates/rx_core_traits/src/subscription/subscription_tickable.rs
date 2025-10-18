use crate::{SubscriptionLike, Tickable};

/// Subscriptions used for an [Observable][crate::Observable], they are
/// [Tickable] for scheduling, and they also must implement [Drop] which is
/// enforced at [Observable::Subscription][crate::Observable::Subscription]
/// because having it here as a super trait would prevent blanket implementing
/// this trait.
pub trait ObservableSubscription: SubscriptionLike + Tickable {}

impl<T> ObservableSubscription for T where T: SubscriptionLike + Tickable {}

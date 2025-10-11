use crate::{SubscriptionLike, Tickable};

// TODO: Rename to simply Subscription or ShellSubscription?
pub trait TickableSubscription: SubscriptionLike + Tickable {}

impl<T> TickableSubscription for T where T: SubscriptionLike + Tickable {}

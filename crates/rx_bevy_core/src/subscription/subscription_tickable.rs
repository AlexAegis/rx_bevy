use crate::{SubscriptionLike, Tickable};

pub trait TickableSubscription: SubscriptionLike + Tickable {}

impl<T> TickableSubscription for T where T: SubscriptionLike + Tickable {}

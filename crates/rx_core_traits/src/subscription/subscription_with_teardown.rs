use crate::{SubscriptionLike, TeardownCollection};

/// Subscriptions that either own resources, or can forward them to something
/// that does.
///
/// For example, this trait is applicable to Subscriptions, and other things
/// that actually own resources (passed in in the form of a
/// [Teardown][crate::Teardown]).
///
/// It's also applicable to Subscribers, types that typically do not own these
/// resources but just forward them downstream until it reaches something that
/// does.
pub trait SubscriptionWithTeardown: SubscriptionLike + TeardownCollection {}

impl<T> SubscriptionWithTeardown for T where T: SubscriptionLike + TeardownCollection {}

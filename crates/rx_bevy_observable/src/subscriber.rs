use crate::{Observer, SubscriptionLike};

/// A [Subscriber] is an [Observer] that is also a [SubscriptionLike], so it
/// can clean itself up upon unsubscribe.
///
/// [Subscriber]s are always owned by something and are never passed as references, hence 'static.
///
/// A struct implementing [Subscriber] should have all their fields as private,
/// as users will never directly interact with a [Subscriber]
pub trait Subscriber: 'static + Observer + SubscriptionLike {}

impl<T> Subscriber for T where T: 'static + Observer + SubscriptionLike {}

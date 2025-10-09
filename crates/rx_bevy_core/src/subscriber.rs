use crate::{Observer, SubscriptionLike, WithContext};

/// # [Subscriber]
///
/// A [Subscriber] is an [Observer] that is also a [SubscriptionLike], so it
/// can clean itself up upon unsubscribe.
///
/// ## For Implementations
///
/// A struct implementing [Subscriber] should have all their fields as private,
/// as users will never directly interact with a [Subscriber].
///
/// ### Inlining
///
/// A subscribers [Observer] functions like `next`, `error` and `complete`
/// that just simply forward the signal to its destination should always
/// be `#[inline]`.
pub trait Subscriber: Observer + SubscriptionLike + WithContext {}

impl<T> Subscriber for T where T: Observer + SubscriptionLike + WithContext {}

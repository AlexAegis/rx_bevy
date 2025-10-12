use crate::SubscriptionLike;

/// # WeakSubscriptionHandle
///
/// These are clonable handles for
/// [ObservableSubscription][crate::ObservableSubscription]s and other
/// handles that own a subscription, allowing them to be unsubscribed from
/// multiple places without preventing them to be dropped, or to
/// allowing other places to erroneously tick it.
///
/// Can be acquired by calling [`downgrade`][ScheduledSubscriptionHandle::downgrade]
/// on a [ScheduledSubscriptionHandle] or on an [UnscheduledSubscriptionHandle].
///
/// ## Note To Implementors
///
/// While this trait is empty I want you to explicitly declare a type meant to
/// be used for this as it has to align with some expected behavior
///
/// - It must not unsubscribe on drop, as these are not owners of the
///   subscription they point to.
/// - It most not be tickable. Only the main "strong" handle can be tickable,
///   and that one is not allowed to be cloned.
pub trait WeakSubscriptionHandle: SubscriptionLike + Clone + Send + Sync {}

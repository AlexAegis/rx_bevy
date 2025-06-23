use crate::{Observer, SubscriptionLike};

/// # [Subscriber]
///
/// A [Subscriber] is an [Observer] that is also a [SubscriptionLike], so it
/// can clean itself up upon unsubscribe.
///
/// [Subscriber]s are always owned by something and are never passed as references, hence 'static.
///
/// A struct implementing [Subscriber] should have all their fields as private,
/// as users will never directly interact with a [Subscriber]
///
/// ## Inlining
///
/// A subscribers [Observer] functions like `next`, `error` and `complete`
/// that just simply forward the signal to its destination should always
/// be `#[inline]`. This results in roughly the same outcome what the `operate`
/// function achieves in `RxJS`, which takes the signal handlers from the
/// destination and wraps only those you define. So, if an error is thrown into
/// a long chain of operators that do not interact with the error signal, the
/// error will go straight to the destination observer with a single `error`
/// call.
pub trait Subscriber: 'static + Observer + SubscriptionLike {
	fn finalize(&mut self);
}

impl<T> Subscriber for T
where
	T: 'static + Observer + SubscriptionLike,
{
	fn finalize(&mut self) {}
}

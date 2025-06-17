/// A subscription is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational, and safe to drop
pub trait Subscription {
	fn unsubscribe(&mut self);

	fn is_closed(&self) -> bool;
}

impl Subscription for () {
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self) {}
}

pub trait SubscriptionInstance {}

/// Maybe a subscription would only be a trait and each type would hav
pub trait Subscription {
	type Instance: SubscriptionInstance;

	fn unsubscribe(&mut self);
}
/* TODO: use a concrete genreic wrapper
impl<T> Drop for Subscription {
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
*/

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

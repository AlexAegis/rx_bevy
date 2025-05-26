pub struct Subscription {}

/// unsubscribe on drop? or drop on unsubscribe? both?
impl Subscription {
	fn add<Finalizer: Fn() -> ()>(finalizer: Finalizer) {}

	/// This is a mutable reference because [Drop] drops with a mutable reference
	/// too.
	pub fn unsubscribe(&mut self) {}
}

impl Drop for Subscription {
	fn drop(&mut self) {
		self.unsubscribe();
	}
}

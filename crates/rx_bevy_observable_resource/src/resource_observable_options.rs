#[derive(Clone)]
pub struct ResourceObservableOptions {
	/// Emit also when the resource was just added. Note that the observable
	/// does **NOT** trigger immediately when the resource is added, but on the
	/// schedule the subscription was made for when it was first observed as
	/// added.
	///
	/// Default: `true`
	pub trigger_on_is_added: bool,
	/// Emit on each tick where the resource was accessed mutably.
	/// Adds don't count here.
	///
	/// Default: `true`
	pub trigger_on_is_changed: bool,
}

impl Default for ResourceObservableOptions {
	fn default() -> Self {
		Self {
			trigger_on_is_added: true,
			trigger_on_is_changed: true,
		}
	}
}

#[derive(Clone, Debug)]
pub struct OnTickObservableOptions {
	/// Whether or not the first emission, `0` should happen on subscribe
	/// or on the first tick after.
	///
	/// Default: `false`
	pub start_on_subscribe: bool,
	/// When larger than 0, one iteration of the iterator will happen every nth
	/// tick, regardless of how long or small that tick was.
	/// When is 0, the entire iterator will be emitted immediately on subscripton.
	///
	/// Default: `1`
	pub emit_at_every_nth_tick: usize,
}

impl Default for OnTickObservableOptions {
	fn default() -> Self {
		Self {
			emit_at_every_nth_tick: 1,
			start_on_subscribe: false,
		}
	}
}

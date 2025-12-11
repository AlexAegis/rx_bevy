#[derive(Debug, Default, Clone)]
pub struct OnTickObservableOptions {
	/// Whether or not the first emission, `0` should happen on subscribe
	/// or on the first tick after.
	///
	/// This means the first emission will happen **immediately** when the
	/// subscribe call happened, which is can be outside of the schedule where
	/// the rest of the emissions will happen.
	pub start_on_subscribe: bool,
	/// When larger than 0, one iteration of the iterator will happen every nth
	/// tick, regardless of how long or small that tick was.
	/// When is 0, the entire iterator will be emitted immediately on subscripton
	pub emit_at_every_nth_tick: usize,
}

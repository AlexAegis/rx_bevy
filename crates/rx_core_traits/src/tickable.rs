use crate::{SubscriptionContext, Tick, WithSubscriptionContext};

#[deprecated]
pub trait Tickable: WithSubscriptionContext {
	/// A channel for push based scheduling, processing ticks issued by
	/// schedulers.
	///
	/// Some operators may produce other, new signals during a tick.
	/// None of the regular operators do anything on a tick but notify it's
	/// downstream of the tick.
	///
	/// ## For implementations
	///
	/// Do not block the propagation of a tick by checking if something is
	/// closed. Even a closed subscriber must forward ticks in case there is
	/// a downstream subscription still expecting the tick.
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	);
}

/// For usecases where the context is not used at all, some convenience
/// functions are provided
pub trait TickableWithDefaultDropContext: Tickable {
	/// Convenience function that uses the default drop context to `tick`
	fn tick_noctx(&mut self, tick: Tick) {
		let mut context = Self::Context::create_context_to_unsubscribe_on_drop();
		self.tick(tick, &mut context);
	}
}

impl<T> TickableWithDefaultDropContext for T where T: Tickable {}

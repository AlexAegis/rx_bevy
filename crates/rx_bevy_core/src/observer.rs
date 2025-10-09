use crate::{SignalContext, Tick, WithContext};

pub trait ObserverInput {
	type In: 'static;
	type InError: 'static;
}

pub trait Observer: ObserverInput + WithContext {
	/// TODO: Maybe rename all contextual functions to xy_with_context and add default implemented functions for a plain next where the context is just the default, but it should disallow overriding the default impl, so maybe on a sealed trait?
	fn next(&mut self, next: Self::In, context: &mut Self::Context);
	fn error(&mut self, error: Self::InError, context: &mut Self::Context);
	fn complete(&mut self, context: &mut Self::Context);

	/// Special fourth channel to process ticks issued by the schedulers.
	/// Some operators may produce other, new signals during a tick.
	/// None of the regular operators do anything on a tick but notify it's
	/// downstream of the tick.
	fn tick(&mut self, tick: Tick, context: &mut Self::Context);
}

/// For usecases where the context is not used at all, some convenience
/// functions are provided
pub trait ObserverWithDefaultDropContext: Observer {
	/// Convenience function that uses the default drop context to `next`
	fn next_noctx(&mut self, next: Self::In) {
		let mut context = Self::Context::create_context_to_unsubscribe_on_drop();
		self.next(next, &mut context);
	}

	/// Convenience function that uses the default drop context to `error`
	fn error_noctx(&mut self, error: Self::InError) {
		let mut context = Self::Context::create_context_to_unsubscribe_on_drop();
		self.error(error, &mut context);
	}

	/// Convenience function that uses the default drop context to `complete`
	fn complete_noctx(&mut self) {
		let mut context = Self::Context::create_context_to_unsubscribe_on_drop();
		self.complete(&mut context);
	}

	/// Convenience function that uses the default drop context to `tick`
	fn tick_noctx(&mut self, tick: Tick) {
		let mut context = Self::Context::create_context_to_unsubscribe_on_drop();
		self.tick(tick, &mut context);
	}
}

impl<T> ObserverWithDefaultDropContext for T where T: Observer {}

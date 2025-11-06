use crate::{SignalBound, SubscriptionContext, WithSubscriptionContext};

pub trait ObserverInput {
	type In: SignalBound;
	type InError: SignalBound;
}

pub trait Observer: ObserverInput + WithSubscriptionContext + Send + Sync {
	/// TODO: Maybe rename all contextual functions to xy_with_context and add default implemented functions for a plain next where the context is just the default, but it should disallow overriding the default impl, so maybe on a sealed trait?
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	);
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	);
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>);
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
}

impl<T> ObserverWithDefaultDropContext for T where T: Observer {}

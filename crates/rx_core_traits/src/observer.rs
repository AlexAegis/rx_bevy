use crate::{Signal, SubscriptionContext, WithSubscriptionContext};

pub trait ObserverInput {
	type In: Signal;
	type InError: Signal;
}

pub trait Observer: ObserverInput + WithSubscriptionContext + Send + Sync {
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

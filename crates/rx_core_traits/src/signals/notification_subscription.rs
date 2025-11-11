use crate::{SubscriptionScheduled, Teardown, Tick, context::SubscriptionContext};

/// Represents a signal event in a materialized form
#[derive(Debug)]
pub enum SubscriptionNotification<Context>
where
	Context: SubscriptionContext,
{
	Unsubscribe,
	/// Add contains an Option of a teardown because cloned versions of a
	/// SubscriptionNotification::Add cannot have a cloned version of a unique
	/// resource it owns. The teardown must be unique.
	Add(Option<Teardown<Context>>),
	Tick(Tick),
}

impl<Context> Clone for SubscriptionNotification<Context>
where
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		match self {
			Self::Tick(tick) => Self::Tick(tick.clone()),
			Self::Unsubscribe => Self::Unsubscribe,
			Self::Add(_) => Self::Add(None), // Must not clone a unique resource
		}
	}
}

pub trait SubscriptionScheduledPushNotificationExtention: SubscriptionScheduled {
	fn push(
		&mut self,
		notification: impl Into<SubscriptionNotification<Self::Context>>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	);
}

impl<T> SubscriptionScheduledPushNotificationExtention for T
where
	T: SubscriptionScheduled,
{
	fn push(
		&mut self,
		notification: impl Into<SubscriptionNotification<Self::Context>>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		match notification.into() {
			SubscriptionNotification::Tick(tick) => self.tick(tick, context),
			SubscriptionNotification::Add(Some(teardown)) => self.add_teardown(teardown, context),
			SubscriptionNotification::Add(None) => {}
			SubscriptionNotification::Unsubscribe => self.unsubscribe(context),
		}
	}
}

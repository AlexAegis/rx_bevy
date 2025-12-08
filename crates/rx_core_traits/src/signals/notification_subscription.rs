use crate::{SubscriptionWithTeardown, Teardown};

/// Represents a signal event in a materialized form
#[derive(Debug)]
pub enum SubscriptionNotification {
	Unsubscribe,
	/// Add contains an Option of a teardown because cloned versions of a
	/// SubscriptionNotification::Add cannot have a cloned version of a unique
	/// resource it owns. The teardown must be unique.
	Add(Option<Teardown>),
}

impl Clone for SubscriptionNotification {
	fn clone(&self) -> Self {
		match self {
			Self::Unsubscribe => Self::Unsubscribe,
			Self::Add(_) => Self::Add(None), // Must not clone a unique resource
		}
	}
}

pub trait SubscriptionLikePushNotificationExtention: SubscriptionWithTeardown {
	fn push(&mut self, notification: impl Into<SubscriptionNotification>);
}

impl<T> SubscriptionLikePushNotificationExtention for T
where
	T: SubscriptionWithTeardown,
{
	fn push(&mut self, notification: impl Into<SubscriptionNotification>) {
		match notification.into() {
			SubscriptionNotification::Add(Some(teardown)) => self.add_teardown(teardown),
			SubscriptionNotification::Add(None) => {}
			SubscriptionNotification::Unsubscribe => self.unsubscribe(),
		}
	}
}

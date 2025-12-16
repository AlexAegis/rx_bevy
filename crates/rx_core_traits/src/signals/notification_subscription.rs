use crate::SubscriptionWithTeardown;

/// Represents a signal event in a materialized form
#[derive(Debug)]
pub enum SubscriptionNotification {
	Unsubscribe,
}

impl Clone for SubscriptionNotification {
	fn clone(&self) -> Self {
		match self {
			Self::Unsubscribe => Self::Unsubscribe,
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
			SubscriptionNotification::Unsubscribe => self.unsubscribe(),
		}
	}
}

use crate::{
	ObserverNotification, Signal, Subscriber, SubscriptionContext, SubscriptionNotification,
	Teardown, Tick,
};

/// Represents all signal events a subscriber can observe in a materialized form
#[derive(Debug)]
pub enum SubscriberNotification<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	Next(In),
	Error(InError),
	Complete,
	Tick(Tick),
	Unsubscribe,
	Add(Option<Teardown<Context>>),
}

impl<In, InError, Context> Clone for SubscriberNotification<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		match self {
			Self::Next(next) => Self::Next(next.clone()),
			Self::Error(error) => Self::Error(error.clone()),
			Self::Complete => Self::Complete,
			Self::Tick(tick) => Self::Tick(*tick),
			Self::Unsubscribe => Self::Unsubscribe,
			Self::Add(_) => Self::Add(None),
		}
	}
}

impl<In, InError, Context> From<ObserverNotification<In, InError>>
	for SubscriberNotification<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	fn from(value: ObserverNotification<In, InError>) -> Self {
		match value {
			ObserverNotification::Next(next) => SubscriberNotification::Next(next),
			ObserverNotification::Error(error) => SubscriberNotification::Error(error),
			ObserverNotification::Complete => SubscriberNotification::Complete,
		}
	}
}

impl<In, InError, Context> From<SubscriptionNotification<Context>>
	for SubscriberNotification<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	fn from(value: SubscriptionNotification<Context>) -> Self {
		match value {
			SubscriptionNotification::Unsubscribe => SubscriberNotification::Unsubscribe,
			SubscriptionNotification::Add(teardown) => SubscriberNotification::Add(teardown),
			SubscriptionNotification::Tick(tick) => SubscriberNotification::Tick(tick),
		}
	}
}

pub trait SubscriberPushNotificationExtention: Subscriber {
	fn push(
		&mut self,
		notification: impl Into<SubscriberNotification<Self::In, Self::InError, Self::Context>>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	);
}

impl<T> SubscriberPushNotificationExtention for T
where
	T: Subscriber,
{
	fn push(
		&mut self,
		notification: impl Into<SubscriberNotification<Self::In, Self::InError, Self::Context>>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		match notification.into() {
			SubscriberNotification::Next(next) => self.next(next, context),
			SubscriberNotification::Error(error) => self.error(error, context),
			SubscriberNotification::Complete => self.complete(context),
			SubscriberNotification::Tick(tick) => self.tick(tick, context),
			SubscriberNotification::Add(Some(teardown)) => self.add_teardown(teardown, context),
			SubscriberNotification::Add(None) => {}
			SubscriberNotification::Unsubscribe => self.unsubscribe(context),
		}
	}
}

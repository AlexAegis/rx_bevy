use crate::{SignalBound, SubjectLike, Subscriber, Teardown, Tick, context::SubscriptionContext};

/// Represents all signal events an observer can observe in a materialized form
#[derive(Debug)]
pub enum ObserverNotification<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	Next(In),
	Error(InError),
	Complete,
}

/// Represents all signal events a subscriber can observe in a materialized form
#[derive(Debug)]
pub enum SubscriberNotification<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	Next(In),
	Error(InError),
	Complete,
	Tick(Tick),
	Unsubscribe,
	Add(Option<Teardown<Context>>),
}

/// Represents a signal event in a materialized form
#[derive(Debug)]
pub enum SubscriptionNotification<Context>
where
	Context: SubscriptionContext,
{
	Unsubscribe,
	Add(Teardown<Context>),
	Tick(Tick),
}

impl<In, InError, Context> TryFrom<SubscriberNotification<In, InError, Context>>
	for ObserverNotification<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	// TODO: Add a real error type
	type Error = ();

	fn try_from(
		value: SubscriberNotification<In, InError, Context>,
	) -> Result<Self, <Self as TryFrom<SubscriberNotification<In, InError, Context>>>::Error> {
		match value {
			SubscriberNotification::Next(next) => Ok(ObserverNotification::Next(next)),
			SubscriberNotification::Error(error) => Ok(ObserverNotification::Error(error)),
			SubscriberNotification::Complete => Ok(ObserverNotification::Complete),
			_ => Err(()),
		}
	}
}

impl<In, InError, Context> From<SubscriptionNotification<Context>>
	for SubscriberNotification<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn from(value: SubscriptionNotification<Context>) -> Self {
		match value {
			SubscriptionNotification::Unsubscribe => SubscriberNotification::Unsubscribe,
			SubscriptionNotification::Add(teardown) => SubscriberNotification::Add(Some(teardown)),
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

pub trait SubjectPushNotificationExtention: SubjectLike {
	fn push(
		&mut self,
		notification: impl Into<ObserverNotification<Self::In, Self::InError>>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	);
}

impl<T> SubjectPushNotificationExtention for T
where
	T: SubjectLike,
{
	fn push(
		&mut self,
		notification: impl Into<ObserverNotification<Self::In, Self::InError>>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		match notification.into() {
			ObserverNotification::Next(next) => self.next(next, context),
			ObserverNotification::Error(error) => self.error(error, context),
			ObserverNotification::Complete => self.complete(context),
		}
	}
}

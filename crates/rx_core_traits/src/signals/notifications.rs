use crate::{SignalBound, Teardown, Tick, context::SubscriptionContext};

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

use bevy_ecs::{entity::Entity, event::Event};
use rx_bevy_core::{SignalBound, SubscriberNotification, Tick};

use crate::BevySubscriberContext;

#[derive(Event, Clone)]
pub enum CommandSubscriberNotification<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	Next(In),
	Error(InError),
	Complete,
	Tick(Tick),
	Unsubscribe,
	Add(Entity),
}

pub trait IntoCommandSubscriberNotification<'world, 'state, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn into_command_subscriber_notification(
		self,
		context: &mut BevySubscriberContext<'world, 'state>,
	) -> CommandSubscriberNotification<In, InError>;
}

impl<'world, 'state, In, InError> IntoCommandSubscriberNotification<'world, 'state, In, InError>
	for SubscriberNotification<In, InError, BevySubscriberContext<'world, 'state>>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn into_command_subscriber_notification(
		self,
		context: &mut BevySubscriberContext<'world, 'state>,
	) -> CommandSubscriberNotification<In, InError> {
		match self {
			SubscriberNotification::Next(next) => CommandSubscriberNotification::Next(next),
			SubscriberNotification::Error(error) => CommandSubscriberNotification::Error(error),
			SubscriberNotification::Complete => CommandSubscriberNotification::Complete,
			SubscriberNotification::Tick(tick) => CommandSubscriberNotification::Tick(tick),
			SubscriberNotification::Unsubscribe => CommandSubscriberNotification::Unsubscribe,
			SubscriberNotification::Add(Some(teardown)) => {
				let teardown_entity = context.spawn_teardown_entity(teardown);
				CommandSubscriberNotification::Add(teardown_entity)
			}
			SubscriberNotification::Add(None) => CommandSubscriberNotification::Unsubscribe,
		}
	}
}

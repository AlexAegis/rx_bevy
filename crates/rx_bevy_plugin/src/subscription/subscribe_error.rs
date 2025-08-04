use bevy_ecs::{
	entity::Entity,
	error::{BevyError, ErrorContext},
};
use bevy_log::error;
use thiserror::Error;

/// Errors that can happen during a [Subscribe] event.
#[derive(Error, Debug)]
pub enum SubscribeError {
	#[error("Tried to subscribe to {0}. But it does not exist on entity {1}.")]
	NotAnObservable(String, Entity),
	#[error("Tried to subscribe to {0}. But it disallows subscriptions from the same entity {1}.")]
	SelfSubscribeDisallowed(String, Entity),
	#[error(
		"Tried to subscribe to a scheduled observable with an unscheduled Subscription! {0} {1}"
	)]
	UnscheduledSubscribeOnScheduledObservable(String, Entity),
	#[error(
		"Tried to subscribe to an unscheduled observable with a scheduled Subscription! {0} {1}"
	)]
	ScheduledSubscribeOnUnscheduledObservable(String, Entity),
}

/// The default error handler just prints out the error as warning
pub(crate) fn default_on_subscribe_error_handler(error: BevyError, error_context: ErrorContext) {
	if let Some(subscribe_error) = error.downcast_ref::<SubscribeError>() {
		error!("{}", subscribe_error);
	} else {
		panic!(
			"Unknown error happened during subscribe. Kind: {}\tName: {}",
			error_context.kind(),
			error_context.name()
		);
	}
}

use rx_core_notification_store::NotificationState;

#[test]
fn should_be_able_to_return_the_current_value() {
	let mut state = NotificationState::<i32>::default();
	assert!(state.get_value().is_none());
	state.next(1);
	assert!(!state.is_waiting());
	assert_eq!(state.get_value(), Some(1).as_ref());
}

#[test]
fn should_be_able_to_take_the_current_value() {
	let mut state = NotificationState::<i32>::default();
	state.next(1);
	assert_eq!(state.take_value(), Some(1));
	assert!(state.is_empty(), "the state should be empty now!");
	assert!(state.is_primed(), "the state should remain primed!");
}

#[test]
fn should_replace_the_nexted_value() {
	let mut state = NotificationState::<i32>::default();
	assert!(state.get_value().is_none());
	state.next(1);
	assert!(!state.is_waiting());
	state.next(2);
	assert_eq!(state.get_value(), Some(2).as_ref())
}

#[test]
fn should_complete() {
	let mut state = NotificationState::<i32>::default();
	state.complete();
	assert!(state.is_completed());
}

#[test]
fn should_error() {
	let mut state = NotificationState::<i32, &'static str>::default();
	state.error("error");
	assert!(state.is_errored());
}

#[test]
fn should_be_able_to_take_the_current_error() {
	let mut state = NotificationState::<i32, &'static str>::default();
	let error = "error";
	state.error(error);
	assert!(state.is_errored());
	assert_eq!(state.take_error(), Some(error));
	assert!(state.is_errored(), "should still count as errored!");
}

#[test]
fn should_unsubscribe() {
	let mut state = NotificationState::<i32>::default();
	state.unsubscribe();
	assert!(state.is_unsubscribed());
}

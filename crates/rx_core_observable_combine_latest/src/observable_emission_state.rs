use std::ops::Deref;

use derive_where::derive_where;
use rx_core_emission_variants::SubscriberState;

/// Stores a single `next` emission from an observable to be used later,
/// along with it's state of being completed/unsubscribed or if it's still
/// waiting for any interaction.
#[derive_where(Debug)]
pub struct ObservableEmissionLastState<T> {
	#[derive_where(skip)]
	value: Option<T>,
	state: SubscriberState,
}

impl<T> Default for ObservableEmissionLastState<T> {
	#[inline]
	fn default() -> Self {
		Self {
			value: None,
			state: SubscriberState::default(),
		}
	}
}

/// Deref is implemented to expose the immutable only api of [SubscriberState]
impl<T> Deref for ObservableEmissionLastState<T> {
	type Target = SubscriberState;

	fn deref(&self) -> &Self::Target {
		&self.state
	}
}

impl<T> ObservableEmissionLastState<T> {
	/// Storing the value, replacing the previous one if there was one.
	/// Marks the state as no longer waiting.
	#[inline]
	pub fn next(&mut self, value: T) {
		self.state.next();
		self.value = Some(value);
	}

	/// Returns the stored value
	#[inline]
	pub fn get(&self) -> Option<&T> {
		self.value.as_ref()
	}

	#[inline]
	pub fn complete(&mut self) {
		self.state.complete();
	}

	#[inline]
	pub fn error(&mut self) {
		self.state.error();
	}

	#[inline]
	pub fn unsubscribe(&mut self) {
		self.state.unsubscribe();
	}
}

#[cfg(test)]
mod test {
	use crate::ObservableEmissionLastState;

	#[test]
	fn should_take_up_a_nexted_value() {
		let mut state = ObservableEmissionLastState::<i32>::default();
		assert!(state.get().is_none());
		state.next(1);
		assert!(!state.is_waiting());
		assert!(matches!(state.get(), Some(1)))
	}

	#[test]
	fn should_replace_the_nexted_value() {
		let mut state = ObservableEmissionLastState::<i32>::default();
		assert!(state.get().is_none());
		state.next(1);
		assert!(!state.is_waiting());
		state.next(2);
		assert!(matches!(state.get(), Some(2)))
	}

	#[test]
	fn should_complete() {
		let mut state = ObservableEmissionLastState::<i32>::default();
		state.complete();
		assert!(state.is_completed());
	}

	#[test]
	fn should_error() {
		let mut state = ObservableEmissionLastState::<i32>::default();
		state.error();
		assert!(state.is_errored());
	}

	#[test]
	fn should_unsubscribe() {
		let mut state = ObservableEmissionLastState::<i32>::default();
		state.unsubscribe();
		assert!(state.is_unsubscribed());
	}
}

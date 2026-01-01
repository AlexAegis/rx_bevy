use core::ops::Deref;

use derive_where::derive_where;
use rx_core_traits::{Never, Signal, SubscriberNotification, SubscriberState};

/// Stores a single `next` emission from an observable to be used later,
/// along with it's state of being completed/unsubscribed or if it's still
/// waiting for any interaction.
#[derive_where(Debug; Out, OutError)]
pub struct NotificationState<Out, OutError = Never>
where
	Out: Signal,
	OutError: Signal,
{
	#[derive_where(skip)]
	value: Option<Out>,
	error: Option<OutError>,
	state: SubscriberState,
}

impl<Out, OutError> Default for NotificationState<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	#[inline]
	fn default() -> Self {
		Self {
			value: None,
			error: None,
			state: SubscriberState::default(),
		}
	}
}

/// Deref is implemented to expose the immutable only api of [SubscriberState]
impl<Out, OutError> Deref for NotificationState<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	type Target = SubscriberState;

	fn deref(&self) -> &Self::Target {
		&self.state
	}
}

impl<Out, OutError> NotificationState<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	/// Storing the value, replacing the previous one if there was one.
	/// Marks the state as no longer waiting.
	#[inline]
	pub fn next(&mut self, value: Out) {
		self.state.next();
		self.value = Some(value);
	}

	/// Returns the stored value
	#[inline]
	pub fn get_value(&self) -> Option<&Out> {
		self.value.as_ref()
	}

	/// Takes the stored value
	#[inline]
	pub fn take_value(&mut self) -> Option<Out> {
		self.value.take()
	}

	/// Emptyness only reflects that there is no actual stored value here, not
	/// that it never was. If the stored value was taken using
	/// [`take_value`][NotificationState::take_value], this will return `true`.
	///
	/// If you want to check that this state has ever seen a `next`, use
	/// [`is_primed`][SubscriberState::is_primed].
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.value.is_none()
	}

	#[inline]
	pub fn error(&mut self, error: OutError) {
		self.state.error();
		self.error = Some(error);
	}

	/// Returns the stored error
	#[inline]
	pub fn get_error(&mut self) -> Option<&OutError> {
		self.error.as_ref()
	}

	/// Takes the stored error
	#[inline]
	pub fn take_error(&mut self) -> Option<OutError> {
		self.error.take()
	}

	#[inline]
	pub fn complete(&mut self) {
		self.state.complete();
	}

	#[inline]
	pub fn unsubscribe(&mut self) {
		if !self.state.is_unsubscribed() {
			self.state.unsubscribe();
		}
	}

	pub fn push(&mut self, notification: SubscriberNotification<Out, OutError>) {
		match notification {
			SubscriberNotification::Next(next) => self.next(next),
			SubscriberNotification::Error(error) => self.error(error),
			SubscriberNotification::Complete => self.complete(),
			SubscriberNotification::Unsubscribe => self.unsubscribe(),
		}
	}
}

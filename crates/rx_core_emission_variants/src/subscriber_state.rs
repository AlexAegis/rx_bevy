use bitflags::bitflags;
use rx_core_traits::{Signal, SubscriberNotification};

bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
	struct SubscriberStateBits: u8 {
		const WAITING = 0b00000001;
		const PRIMED = 0b00000010;
		const COMPLETED = 0b00000100;
		const ERRORED = 0b00001000;
		const UNSUBSCRIBED = 0b00010000;
	}
}

impl Default for SubscriberStateBits {
	#[inline]
	fn default() -> Self {
		SubscriberStateBits::WAITING
	}
}

/// Stores the possible states a subscriber can be in.
/// Debug asserts ensure that no invalid state changes happen, like
/// changing a completed state into an errored one, or trying to call
/// unsubscribe twice.
///
/// By default it is in the "waiting" state and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SubscriberState {
	state: SubscriberStateBits,
}

impl SubscriberState {
	/// Is considered "waiting" before anything else could happen to it.
	///
	/// This flag starts as `true` and cannot be set. Once `false`, stays
	/// `false`.
	#[inline]
	pub fn is_waiting(&self) -> bool {
		self.state.contains(SubscriberStateBits::WAITING)
	}

	/// Is considered "primed" when it had seen at least one `next` calls.
	///
	/// This flag cannot be unset, once `true`, stays `true`.
	#[inline]
	pub fn is_primed(&self) -> bool {
		self.state.contains(SubscriberStateBits::PRIMED)
	}

	/// Is considered "completed" once it is marked as completed.
	/// After which, only `unsubscribe` can be called, everything else will
	/// panic in debug builds.
	///
	/// This flag cannot be unset, once `true`, stays `true`.
	#[inline]
	pub fn is_completed(&self) -> bool {
		self.state.contains(SubscriberStateBits::COMPLETED)
	}

	#[inline]
	pub fn is_completed_but_not_primed(&self) -> bool {
		self.is_completed() && !self.is_primed()
	}

	/// Is "errored" once it is marked as errored.
	/// After which, only `unsubscribe` can be called, everything else will
	/// panic in debug builds.
	///
	/// This flag cannot be unset, once `true`, stays `true`.
	#[inline]
	pub fn is_errored(&self) -> bool {
		self.state.contains(SubscriberStateBits::ERRORED)
	}

	/// Is "unsubscribed" once it is marked as unsubscribed.
	/// After which, only `unsubscribe` can be called, everything else will
	/// panic in debug builds.
	///
	/// This flag cannot be unset, once `true`, stays `true`.
	#[inline]
	pub fn is_unsubscribed(&self) -> bool {
		self.state.contains(SubscriberStateBits::UNSUBSCRIBED)
	}

	/// It's considered "closed" once it's either completed, errored or
	/// unsubscribed.
	///
	/// This flag cannot be unset, once `true`, stays `true`.
	#[inline]
	pub fn is_closed(&self) -> bool {
		self.is_unsubscribed() || self.is_completed() || self.is_errored()
	}

	/// True when closed, but also primed! Useful to see if something has
	/// useful value, and will definitely not get a new one.
	#[inline]
	pub fn is_closed_but_primed(&self) -> bool {
		self.is_closed() && self.is_primed()
	}

	/// True if it's closed but hadn't received a next call.
	/// Usefule to see if something does not have a useful value, and will
	/// definitely not have one.
	#[inline]
	pub fn is_closed_but_not_primed(&self) -> bool {
		self.is_closed() && !self.is_primed()
	}

	#[inline]
	pub fn is_closed_but_not_primed_and_not_completed(&self) -> bool {
		self.is_closed() && !self.is_primed() && !self.is_completed()
	}

	#[inline]
	pub fn is_closed_but_not_completed(&self) -> bool {
		(self.is_unsubscribed() || self.is_errored()) && !self.is_completed()
	}

	#[inline]
	pub fn is_closed_but_not_completed_and_primed(&self) -> bool {
		self.is_closed_but_not_completed() && self.is_primed()
	}

	/// Marks it as no longer waiting, and primed.
	///
	/// It will panic in debug builds when called from an already closed state.
	#[inline]
	pub fn next(&mut self) {
		debug_assert!(
			!self.is_closed(),
			"It should not be possible that an already closed observable nexts!"
		);

		self.state.remove(SubscriberStateBits::WAITING);
		self.state.insert(SubscriberStateBits::PRIMED);
	}

	/// Marks it as completed and no longer waiting.
	///
	/// It will panic in debug builds when called from an already closed state.
	#[inline]
	pub fn complete(&mut self) {
		debug_assert!(
			!self.is_closed(),
			"It should not be possible that an already closed observable completes!"
		);

		self.state.remove(SubscriberStateBits::WAITING);
		self.state.insert(SubscriberStateBits::COMPLETED);
	}

	/// Marks it as errored and no longer waiting.
	///
	/// It will panic in debug builds when called from an already closed state.
	#[inline]
	pub fn error(&mut self) {
		debug_assert!(
			!self.is_closed(),
			"It should not be possible that an already closed observable errors!"
		);

		self.state.remove(SubscriberStateBits::WAITING);
		self.state.insert(SubscriberStateBits::ERRORED);
	}

	/// Marks it as unsubscribed and no longer waiting.
	///
	/// It will panic in debug builds when called from an already unsubscribed
	/// state.
	#[inline]
	pub fn unsubscribe(&mut self) {
		debug_assert!(
			!self.is_unsubscribed(),
			"It should not be possible that an already unsubscribed observable unsubscribes!"
		);

		self.state.remove(SubscriberStateBits::WAITING);
		self.state.insert(SubscriberStateBits::UNSUBSCRIBED);
	}

	pub fn update_with_notification<In, InError>(
		&mut self,
		notification: &SubscriberNotification<In, InError>,
	) where
		In: Signal,
		InError: Signal,
	{
		match notification {
			SubscriberNotification::Unsubscribe => self.unsubscribe(),
			SubscriberNotification::Complete => self.complete(),
			SubscriberNotification::Error(_) => self.error(),
			SubscriberNotification::Next(_) => self.next(),
			SubscriberNotification::Add(_) => {}
		}
	}

	pub fn update_with_notification_would_be_invalid<In, InError>(
		&mut self,
		notification: &SubscriberNotification<In, InError>,
	) -> bool
	where
		In: Signal,
		InError: Signal,
	{
		match notification {
			SubscriberNotification::Unsubscribe => self.is_unsubscribed(),
			SubscriberNotification::Complete
			| SubscriberNotification::Error(_)
			| SubscriberNotification::Next(_) => self.is_closed(),
			SubscriberNotification::Add(_) => false,
		}
	}
}

#[cfg(test)]
mod test {
	use crate::SubscriberState;
	use rx_core_testing::mute_panic;

	#[test]
	fn it_should_be_waiting_by_default() {
		assert!(SubscriberState::default().is_waiting());
	}

	mod next {
		use super::*;

		#[test]
		fn it_should_not_be_waiting_once_nexted() {
			let mut state = SubscriberState::default();
			state.next();
			assert!(!state.is_waiting());
			assert!(!state.is_closed());
			state.next();
			state.next();
			assert!(!state.is_waiting());
			assert!(!state.is_closed());
		}

		#[test]
		#[should_panic]
		fn it_should_panic_when_nexting_after_complete() {
			mute_panic(|| {
				let mut state = SubscriberState::default();
				state.complete();
				state.next();
			});
		}

		#[test]
		#[should_panic]
		fn it_should_panic_when_nexting_after_error() {
			mute_panic(|| {
				let mut state = SubscriberState::default();
				state.error();
				state.next();
			});
		}

		#[test]
		#[should_panic]
		fn it_should_panic_when_nexting_after_unsubscribe() {
			mute_panic(|| {
				let mut state = SubscriberState::default();
				state.unsubscribe();
				state.next();
			});
		}
	}

	mod complete {
		use super::*;

		#[test]
		fn it_should_not_be_waiting_once_completed() {
			let mut state = SubscriberState::default();
			state.complete();
			assert!(state.is_completed());
			assert!(!state.is_waiting());
		}

		#[test]
		#[should_panic]
		fn it_should_panic_when_completing_twice() {
			mute_panic(|| {
				let mut state = SubscriberState::default();
				state.complete();
				state.complete();
			});
		}

		#[test]
		#[should_panic]
		fn it_should_panic_when_completing_after_error() {
			mute_panic(|| {
				let mut state = SubscriberState::default();
				state.error();
				state.complete();
			});
		}

		#[test]
		#[should_panic]
		fn it_should_panic_when_completing_after_unsubscribe() {
			mute_panic(|| {
				let mut state = SubscriberState::default();
				state.unsubscribe();
				state.complete();
			});
		}
	}

	mod error {
		use super::*;

		#[test]
		fn it_should_not_be_waiting_once_errored() {
			let mut state = SubscriberState::default();
			state.error();
			assert!(state.is_errored());
			assert!(!state.is_waiting());
		}

		#[test]
		#[should_panic]
		fn it_should_panic_when_erroring_after_complete() {
			mute_panic(|| {
				let mut state = SubscriberState::default();
				state.complete();
				state.error();
			});
		}

		#[test]
		#[should_panic]
		fn it_should_panic_when_erroring_twice() {
			mute_panic(|| {
				let mut state = SubscriberState::default();
				state.error();
				state.error();
			});
		}

		#[test]
		#[should_panic]
		fn it_should_panic_when_erroring_after_unsubscribe() {
			mute_panic(|| {
				let mut state = SubscriberState::default();
				state.unsubscribe();
				state.error();
			});
		}
	}

	mod unsubscribe {
		use super::*;

		#[test]
		fn it_should_not_be_waiting_once_unsubscribed() {
			let mut state = SubscriberState::default();
			state.unsubscribe();
			assert!(state.is_unsubscribed());
			assert!(!state.is_waiting());
		}

		#[test]
		fn it_can_unsubscribe_after_completed() {
			let mut state = SubscriberState::default();
			state.complete();
			state.unsubscribe();
			assert!(state.is_completed());
			assert!(state.is_unsubscribed());
			assert!(!state.is_waiting());
		}

		#[test]
		fn it_can_unsubscribe_after_errored() {
			let mut state = SubscriberState::default();
			state.error();
			state.unsubscribe();
			assert!(state.is_errored());
			assert!(state.is_unsubscribed());
			assert!(!state.is_waiting());
		}

		#[test]
		#[should_panic]
		fn it_should_panic_when_unsubscribed_twice() {
			mute_panic(|| {
				let mut state = SubscriberState::default();
				state.unsubscribe();
				state.unsubscribe();
			});
		}
	}
}

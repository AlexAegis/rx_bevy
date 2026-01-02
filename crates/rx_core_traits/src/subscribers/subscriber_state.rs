use crate::{Signal, SubscriberNotification};
use bitflags::bitflags;

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

/// Stores the possible states a subscriber can be in. Useful to track the
/// state of an unknown upstream source of signals based only on those signals.
///
/// Debug asserts ensure that no invalid state changes happen, like
/// changing a completed state into an errored one, or trying to call
/// unsubscribe twice.
///
/// By default it's in the "waiting" state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SubscriberState {
	state: SubscriberStateBits,
}

impl SubscriberState {
	/// Is considered "waiting" when nothing had happened yet.
	///
	/// This flag starts as `true` and cannot be set. Once `false`, stays
	/// `false`.
	#[inline]
	pub fn is_waiting(&self) -> bool {
		self.state.contains(SubscriberStateBits::WAITING)
	}

	/// Is considered "primed" when it had seen at least one `next` call.
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

	/// True when the state was completed without ever seeing a single `next`!
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

	/// It's considered "closed" once it's either unsubscribed, or finished,
	/// meaning it either completed or errored.
	///
	/// This flag cannot be unset, once `true`, stays `true`.
	#[inline]
	pub fn is_closed(&self) -> bool {
		self.is_unsubscribed() || self.is_finished()
	}

	/// It's considered "finished" once it's either completed or errored.
	///
	/// This flag cannot be unset, once `true`, stays `true`.
	#[inline]
	pub fn is_finished(&self) -> bool {
		self.is_completed() || self.is_errored()
	}

	/// True when closed, but also primed! Useful to see if something has
	/// useful value, and will definitely not get a new one.
	#[inline]
	pub fn is_closed_but_primed(&self) -> bool {
		self.is_closed() && self.is_primed()
	}

	/// True if it's closed but hadn't received a next call.
	/// Useful to see if something never had a useful value, and will
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

	/// Marks it as unsubscribed and no longer waiting.
	///
	/// Does the same thing as `unsubscribe` but it will never panic even if it
	/// was already marked as unsubscribed.
	///
	/// Unlike with complete/error, which should definitely never happen twice,
	/// sometimes the steps to unsubscribe has to be performed differently,
	/// when erroring/completing, which may or may not happen before doing an
	/// unsubscribe.
	///
	/// Therefore this method is more like a "make sure it's closed" kind of
	/// method, but one should always prefer using `unsubscribe` here on
	/// [SubscriberState], to catch double unsubscribes.
	#[inline]
	pub fn unsubscribe_if_not_already(&mut self) {
		self.state.remove(SubscriberStateBits::WAITING);
		self.state.insert(SubscriberStateBits::UNSUBSCRIBED);
	}

	/// Applies this notification to the state.
	///
	/// It will panic in debug builds when an invalid state change is attempted.
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
		}
	}

	/// Try to avoid using this function! It's an escape hatch, and a tool to
	/// help narrow down problems.
	///
	/// If using this check before updating the state is what makes your logic
	/// work, it means you're double updating with the same notification.
	/// Or upstream is incorrect by sending the same signal multiple times, in
	/// which case the problem is not you.
	///
	/// Rest assured these invalid update panics only happen in debug builds!
	pub fn notification_matches_state<In, InError>(
		&mut self,
		notification: &SubscriberNotification<In, InError>,
	) -> bool
	where
		In: Signal,
		InError: Signal,
	{
		match notification {
			SubscriberNotification::Unsubscribe => self.is_unsubscribed(),
			SubscriberNotification::Complete => self.is_completed(),
			SubscriberNotification::Error(_) => self.is_errored(),
			SubscriberNotification::Next(_) => self.is_primed(),
		}
	}

	/// Try to avoid using this function! It's an escape hatch, and a tool to
	/// help narrow down problems.
	///
	/// If you haven't yet, try using `notification_matches_state` as your
	/// safety check before reaching for this function, that one can reveal a
	/// narrower problem.
	///
	/// If using this check before updating the state is what makes your logic
	/// work, it means you're either double updating with the same notification,
	/// or you apply a notification that is not a correct state change.
	///
	/// > True for all sources of signals:
	/// > Zero or more Next notifications come first, after which up to 1
	/// > Error or Complete signal, and then finally an Unsubscribe signal, and
	/// > then nothing else.
	///
	/// Or upstream is incorrect by sending the same signal multiple times, in
	/// which case the problem is not you.
	///
	/// Rest assured these invalid update panics only happen in debug builds!
	pub fn update_with_notification_would_be_invalid<In, InError>(
		&self,
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
		}
	}
}

#[cfg(test)]
mod test {
	use crate::SubscriberState;

	fn mute_panic<R>(fun: impl FnOnce() -> R) -> R {
		let hook = std::panic::take_hook();
		std::panic::set_hook(Box::new(|_| {}));
		let result = fun();
		std::panic::set_hook(hook);
		result
	}

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

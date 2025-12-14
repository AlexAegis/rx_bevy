use std::ops::Deref;

/// A simple newtype to serve as the `is_closed` flag in subscriptions.
/// It makes sure that once it's closed it can't be opened again.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubscriptionClosedFlag {
	is_closed: bool,
}

impl Deref for SubscriptionClosedFlag {
	type Target = bool;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.is_closed
	}
}

impl Default for SubscriptionClosedFlag {
	#[inline]
	fn default() -> Self {
		Self::new_opened()
	}
}

impl From<bool> for SubscriptionClosedFlag {
	#[inline]
	fn from(is_closed: bool) -> Self {
		Self::new(is_closed)
	}
}

impl Drop for SubscriptionClosedFlag {
	fn drop(&mut self) {
		if !self.is_closed() {
			// This debug assertion helps to ensure that where it is used, it is
			// explicitly unsubscribed from, instead of relying on drop
			// unsubscribing them, which is not guaranteed to happen in all cases.
			debug_assert!(
				false,
				"SubscriptionClosedFlag was dropped without closing it!"
			)
		}
	}
}

impl SubscriptionClosedFlag {
	#[inline]
	fn new(is_closed: bool) -> Self {
		Self { is_closed }
	}

	#[inline]
	pub fn new_opened() -> Self {
		Self::new(false)
	}

	#[inline]
	pub fn new_closed() -> Self {
		Self::new(true)
	}

	#[inline]
	pub fn is_closed(&self) -> bool {
		self.is_closed
	}

	#[inline]
	pub fn close(&mut self) {
		self.is_closed = true
	}
}

#[cfg(test)]
mod test {
	use crate::SubscriptionClosedFlag;

	#[test]
	fn it_can_be_created_from_a_bool() {
		let flag: SubscriptionClosedFlag = true.into();
		assert!(flag.is_closed());
	}

	#[test]
	fn when_created_as_open_it_returns_its_initial_state() {
		let mut flag = SubscriptionClosedFlag::new_opened();
		assert!(!flag.is_closed());
		// Close because of drop protection
		flag.close();
	}

	#[test]
	fn when_created_as_closed_it_returns_its_initial_state() {
		let flag = SubscriptionClosedFlag::new_closed();
		assert!(flag.is_closed());
	}

	#[test]
	fn when_closed_its_state_changes() {
		let mut flag = SubscriptionClosedFlag::new_opened();
		assert!(!flag.is_closed());
		flag.close();
		assert!(flag.is_closed());
	}

	/// Verifies:
	/// - RX_WHATS_CLOSED_STAYS_CLOSED
	#[test]
	fn successive_closes_do_not_change_its_state() {
		let mut flag = SubscriptionClosedFlag::new_opened();
		assert!(!flag.is_closed());
		flag.close();
		assert!(flag.is_closed());
		flag.close();
		assert!(flag.is_closed());
	}
}

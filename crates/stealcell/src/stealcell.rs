use std::ops::{Deref, DerefMut};

const ALREADY_STOLEN: &str = "value already stolen!";

/// Allows you to "steal" its value, taking complete ownership over it, and you
/// must pinky-promise to return it. Non-returned values will panic once dropped!
pub struct StealCell<T> {
	value: Option<T>,
}

/// A value stolen from a [StealCell]. If you accidentally drop it before
/// returning it where it belongs, it will panic!
pub struct Stolen<T> {
	/// Starts out as Some, becomes None once returned.
	/// If not returned, panics!
	value: Option<T>,
}

impl<T> StealCell<T> {
	pub fn new(value: T) -> Self {
		Self { value: Some(value) }
	}

	pub fn steal(&mut self) -> Stolen<T> {
		let value = self.value.take().expect(ALREADY_STOLEN);
		Stolen { value: Some(value) }
	}

	pub fn is_stolen(&self) -> bool {
		self.value.is_none()
	}

	pub fn return_stolen(&mut self, mut stolen: Stolen<T>) {
		assert!(
			self.value.is_none(),
			"trying to return a stolen value, but this cell is not empty!"
		);
		assert!(
			stolen.value.is_some(),
			"trying to return a stolen value, but it was already returned!"
		);
		self.value = Some(stolen.value.take().unwrap());
	}

	pub fn get(&self) -> &T {
		self.value.as_ref().expect(ALREADY_STOLEN)
	}

	pub fn get_mut(&mut self) -> &mut T {
		self.value.as_mut().expect(ALREADY_STOLEN)
	}

	pub fn as_deref(&self) -> &T::Target
	where
		T: std::ops::Deref,
	{
		self.value.as_ref().expect(ALREADY_STOLEN).deref()
	}

	pub fn as_deref_mut(&mut self) -> &mut T::Target
	where
		T: std::ops::DerefMut,
	{
		self.value.as_mut().expect(ALREADY_STOLEN).deref_mut()
	}
}

impl<T> Stolen<T> {
	pub fn get_mut(&mut self) -> &mut T {
		self.value.as_mut().unwrap()
	}
}

impl<T> Deref for Stolen<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.value.as_ref().unwrap()
	}
}

impl<T> DerefMut for Stolen<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.value.as_mut().unwrap()
	}
}

impl<T> Drop for Stolen<T> {
	fn drop(&mut self) {
		if self.value.is_some() {
			panic!("You've lost a stolen value without returning it first!");
		}
	}
}

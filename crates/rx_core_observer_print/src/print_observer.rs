use std::{fmt::Debug, marker::PhantomData};

use rx_core_common::{
	Never, PhantomInvariant, RxObserver, Signal, SubscriptionData, SubscriptionLike, Teardown,
	TeardownCollection,
};
use rx_core_macro_observer_derive::RxObserver;

/// A simple observer that prints out received values using [std::fmt::Debug]
#[derive(RxObserver)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_upgrades_to(self)]
pub struct PrintObserver<In, InError = Never>
where
	In: Signal + Debug,
	InError: Signal + Debug,
{
	prefix: Option<&'static str>,
	teardown: SubscriptionData,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> Clone for PrintObserver<In, InError>
where
	In: Signal + Debug,
	InError: Signal + Debug,
{
	fn clone(&self) -> Self {
		Self {
			prefix: self.prefix,
			teardown: SubscriptionData::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> PrintObserver<In, InError>
where
	In: Signal + Debug,
	InError: Signal + Debug,
{
	pub fn new(message: &'static str) -> Self {
		Self {
			prefix: Some(message),
			teardown: SubscriptionData::default(),
			_phantom_data: PhantomData,
		}
	}

	fn get_prefix(&self) -> String {
		self.prefix
			.map(|prefix| format!("{prefix} - "))
			.unwrap_or_default()
	}
}

impl<In, InError> Default for PrintObserver<In, InError>
where
	In: Signal + Debug,
	InError: Signal + Debug,
{
	fn default() -> Self {
		Self {
			prefix: None,
			teardown: SubscriptionData::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> RxObserver for PrintObserver<In, InError>
where
	In: Signal + Debug,
	InError: Signal + Debug,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		println!("{}next: {:?}", self.get_prefix(), next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		println!("{}error: {:?}", self.get_prefix(), error);
		self.unsubscribe();
	}

	#[inline]
	fn complete(&mut self) {
		println!("{}completed", self.get_prefix());
		self.unsubscribe();
	}
}

impl<In, InError> SubscriptionLike for PrintObserver<In, InError>
where
	In: Signal + Debug,
	InError: Signal + Debug,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.teardown.unsubscribe();
			println!("{}unsubscribed", self.get_prefix());
		}
	}
}

impl<In, InError> TeardownCollection for PrintObserver<In, InError>
where
	In: Signal + Debug,
	InError: Signal + Debug,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.teardown.add_teardown(teardown);
	}
}

impl<In, InError> Drop for PrintObserver<In, InError>
where
	In: Signal + Debug,
	InError: Signal + Debug,
{
	fn drop(&mut self) {
		// Perform the teardown, but do not print.
		self.teardown.unsubscribe();
	}
}

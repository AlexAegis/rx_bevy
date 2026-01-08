use std::num::NonZero;

use derive_where::derive_where;

use rx_core_common::{RxObserver, Signal, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive_where(Debug)]
#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct BufferCountSubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = Vec<In>>,
{
	#[destination]
	#[derive_where(skip)]
	destination: Destination,
	#[derive_where(skip(Debug))]
	buffer: Option<Vec<In>>,
	buffer_size: NonZero<usize>,
}

impl<In, Destination> BufferCountSubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = Vec<In>>,
{
	pub fn new(destination: Destination, buffer_size: NonZero<usize>) -> Self {
		Self {
			buffer: None,
			destination,
			buffer_size,
		}
	}

	#[inline]
	fn eject_buffer(&mut self) {
		if let Some(buffer) = self.buffer.take() {
			self.destination.next(buffer);
		}
	}
}

impl<In, Destination> RxObserver for BufferCountSubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = Vec<In>>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.buffer.get_or_insert_default().push(next);

		if self
			.buffer
			.as_ref()
			.is_some_and(|buffer| buffer.len() == self.buffer_size.into())
		{
			self.eject_buffer();
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.eject_buffer();
		self.destination.complete();
	}
}

use std::collections::VecDeque;

use derive_where::derive_where;
use rx_core_common::Observable;
use rx_core_subscriber_higher_order::HigherOrderSubscriberStateConditions;

#[derive_where(Default)]
pub(crate) struct ConcurrentSubscriberQueue<InnerObservable>
where
	InnerObservable: Observable,
{
	pub(crate) queue: VecDeque<InnerObservable>,
}

impl<InnerObservable> HigherOrderSubscriberStateConditions
	for ConcurrentSubscriberQueue<InnerObservable>
where
	InnerObservable: Observable,
{
	#[inline]
	fn can_downstream_complete(&self) -> bool {
		self.queue.is_empty()
	}

	#[inline]
	fn can_downstream_unsubscribe(&self) -> bool {
		self.queue.is_empty()
	}

	#[inline]
	fn on_upstream_error(&mut self) {
		self.queue.drain(..);
	}

	#[inline]
	fn on_downstream_error(&mut self) {
		self.queue.drain(..);
	}
}

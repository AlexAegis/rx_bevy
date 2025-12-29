use std::{collections::VecDeque, num::NonZero};

use rx_core_traits::Observable;

pub(crate) struct ConcurrentSubscriberState<InnerObservable>
where
	InnerObservable: Observable,
{
	pub(crate) concurrency_limit: NonZero<usize>,
	pub(crate) non_completed_subscriptions: usize,
	pub(crate) non_unsubscribed_subscriptions: usize,
	pub(crate) queue: VecDeque<InnerObservable>,
	pub(crate) upstream_completed: bool, // TODO: Prime candidate for some bitpacking
	pub(crate) upstream_errored: bool,
	pub(crate) upstream_unsubscribed: bool,
	pub(crate) downstream_completed: bool,
	pub(crate) downstream_unsubscribed: bool,
}

impl<InnerObservable> ConcurrentSubscriberState<InnerObservable>
where
	InnerObservable: Observable,
{
	pub(crate) fn new(concurrency_limit: NonZero<usize>) -> Self {
		Self {
			non_completed_subscriptions: 0,
			non_unsubscribed_subscriptions: 0,
			concurrency_limit,
			queue: VecDeque::new(),
			upstream_completed: false,
			upstream_unsubscribed: false,
			upstream_errored: false,
			downstream_completed: false,
			downstream_unsubscribed: false,
		}
	}

	pub(crate) fn can_downstream_complete(&self) -> bool {
		self.queue.is_empty()
			&& self.non_completed_subscriptions == 0
			&& self.upstream_completed
			&& !self.downstream_completed
	}

	pub(crate) fn can_downstream_unsubscribe(&self) -> bool {
		self.queue.is_empty()
			&& self.non_unsubscribed_subscriptions == 0
			&& self.upstream_unsubscribed
			&& !self.downstream_unsubscribed
	}

	pub(crate) fn error(&mut self) {
		self.queue.drain(..);
		self.upstream_errored = true;
		self.upstream_unsubscribed = true;
		self.downstream_unsubscribed = true;
	}
}

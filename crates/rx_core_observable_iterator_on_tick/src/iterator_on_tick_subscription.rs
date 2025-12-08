use std::{
	iter::Peekable,
	sync::{Arc, Mutex},
	time::Duration,
};

use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Never, Scheduler, SchedulerScheduleTaskExtension, Signal, Subscriber, SubscriptionData,
	SubscriptionLike, TaskContextItem, TaskOwnerId, Teardown, TeardownCollection,
};

use crate::observable::OnTickObservableOptions;

struct OnTickIteratorState<Iterator>
where
	Iterator: IntoIterator,
	Iterator::Item: Signal,
{
	last_now_observed: Duration,
	observed_ticks: usize,
	peekable_iterator: Peekable<Iterator::IntoIter>,
}

#[derive(RxSubscription)]
pub struct OnTickIteratorSubscription<Iterator, S>
where
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	S: Scheduler,
{
	options: OnTickObservableOptions<S>,
	destination: Arc<Mutex<dyn Subscriber<In = Iterator::Item, InError = Never> + Send + Sync>>,
	owner_id: TaskOwnerId,
	teardown: SubscriptionData,
}

impl<Iterator, S> OnTickIteratorSubscription<Iterator, S>
where
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	S: Scheduler,
{
	pub fn new(
		mut destination: impl Subscriber<In = Iterator::Item, InError = Never> + 'static,
		iterator: Iterator::IntoIter,
		options: OnTickObservableOptions<S>,
	) -> Self {
		let mut peekable_iterator = iterator.peekable();

		let mut state = OnTickIteratorState {
			last_now_observed: Duration::from_millis(0),
			observed_ticks: 0,
			peekable_iterator,
		};

		if options.start_on_subscribe
			&& let Some(value) = peekable_iterator.next()
		{
			destination.next(value);
		}

		let destination = Arc::new(Mutex::new(destination));

		let owner_id = {
			let scheduler = options.scheduler.get_scheduler();
			let owner_id = scheduler.generate_owner_id();
			scheduler.schedule_repeated_task(
				move |_, context| {
					let is_new_tick = {
						let now = context.now();
						let diff = state.last_now_observed == now;
						state.last_now_observed = now;
						diff
					};

					Ok(())
				},
				Duration::from_nanos(0),
				false,
				owner_id,
			);

			state.observed_ticks += 1;

			if state.emit_at_every_nth_tick != 0
				&& state
					.observed_ticks
					.is_multiple_of(state.emit_at_every_nth_tick)
				&& let Some(value) = state.peekable_iterator.next()
			{
				state.observed_ticks = 0; // Reset to avoid overflow
				state.destination.next(value);
				if self.peekable_iterator.peek().is_none() {
					self.destination.complete();
					self.unsubscribe();
				}
			}

			owner_id
		};

		OnTickIteratorSubscription {
			options,
			destination,
			owner_id,
			teardown: SubscriptionData::default(),
		}
	}
}

impl<Iterator, S> SubscriptionLike for OnTickIteratorSubscription<Iterator, S>
where
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	S: Scheduler,
{
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.options.scheduler.get_scheduler().cancel(self.owner_id);
		if !self.teardown.is_closed() {
			self.destination.unsubscribe();
			self.teardown.unsubscribe();
		}
	}
}

impl<Iterator, S> TeardownCollection for OnTickIteratorSubscription<Iterator, S>
where
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	S: Scheduler,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		self.teardown.add_teardown(teardown);
	}
}

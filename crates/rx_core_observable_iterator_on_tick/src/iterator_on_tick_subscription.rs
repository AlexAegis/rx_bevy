use std::{
	iter::Peekable,
	marker::PhantomData,
	sync::{Arc, Mutex},
	time::Duration,
};

use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Never, Observer, Scheduler, SchedulerScheduleTaskExtension, Signal, Subscriber,
	SubscriptionLike, TaskCancellationId, TaskContext, Teardown, TeardownCollection, TickResult,
};

use crate::observable::OnTickObservableOptions;

struct OnTickIteratorState<Destination, Iterator>
where
	Destination: Subscriber<In = Iterator::Item, InError = Never>,
	Iterator: IntoIterator,
	Iterator::Item: Signal,
{
	now: Duration,
	observed_ticks: usize,
	emit_at_every_nth_tick: usize,
	destination: Arc<Mutex<Destination>>,
	peekable_iterator: Peekable<Iterator::IntoIter>,
}

#[derive(RxSubscription)]
pub struct OnTickIteratorSubscription<Destination, Iterator, S>
where
	Destination: Subscriber<In = Iterator::Item, InError = Never>,
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	S: Scheduler,
{
	options: OnTickObservableOptions<S>,
	destination: Arc<Mutex<Destination>>,
	owner_id: Option<TaskCancellationId>,
	_phantom_data: PhantomData<fn(Iterator) -> Iterator>,
}

impl<Destination, Iterator, S> OnTickIteratorSubscription<Destination, Iterator, S>
where
	Destination: 'static + Subscriber<In = Iterator::Item, InError = Never>,
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	Iterator::IntoIter: 'static + Send + Sync,
	S: Scheduler,
{
	pub fn new(
		destination: Destination,
		iterator: Iterator::IntoIter,
		mut options: OnTickObservableOptions<S>,
	) -> Self {
		let peekable_iterator = iterator.peekable();

		let destination = Arc::new(Mutex::new(destination));

		if options.emit_at_every_nth_tick == 0 {
			return OnTickIteratorSubscription {
				options,
				destination,
				owner_id: None,
				_phantom_data: PhantomData,
			};
		}

		let owner_id = {
			let mut scheduler = options.scheduler.lock();
			let owner_id = scheduler.generate_cancellation_id();

			let mut state = OnTickIteratorState::<Destination, Iterator> {
				now: Duration::from_millis(0),
				observed_ticks: 0,
				emit_at_every_nth_tick: options.emit_at_every_nth_tick,
				peekable_iterator,
				destination: destination.clone(),
			};

			scheduler.schedule_repeated_task(
				move |_, context| {
					// TODO: Some guarantees are needed on re-runnability. maybe ensure every task is ticked at most once?
					let is_new_tick = {
						let now = context.now();
						let diff = state.now == now;
						state.now = now;
						diff
					};

					if !is_new_tick {
						return TickResult::Pending;
					}

					state.observed_ticks += 1;

					if state.emit_at_every_nth_tick != 0
						&& !state.destination.is_closed()
						&& state
							.observed_ticks
							.is_multiple_of(state.emit_at_every_nth_tick)
						&& let Some(value) = state.peekable_iterator.next()
					{
						state.observed_ticks = 0;
						state.destination.next(value);
						if state.peekable_iterator.peek().is_none() {
							state.destination.complete();
							state.destination.unsubscribe();
							TickResult::Done
						} else {
							TickResult::Pending
						}
					} else {
						TickResult::Pending
					}
				},
				Duration::from_nanos(0),
				options.start_on_subscribe,
				owner_id,
			);

			owner_id
		};

		OnTickIteratorSubscription {
			options,
			destination,
			owner_id: Some(owner_id),
			_phantom_data: PhantomData,
		}
	}
}

impl<Destination, Iterator, S> SubscriptionLike
	for OnTickIteratorSubscription<Destination, Iterator, S>
where
	Destination: Subscriber<In = Iterator::Item, InError = Never>,
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	S: Scheduler,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		if let Some(owner_id) = self.owner_id {
			self.options.scheduler.lock().cancel(owner_id);
		}
		self.destination.unsubscribe();
	}
}

impl<Destination, Iterator, S> TeardownCollection
	for OnTickIteratorSubscription<Destination, Iterator, S>
where
	Destination: Subscriber<In = Iterator::Item, InError = Never>,
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	S: Scheduler,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		self.destination.add_teardown(teardown);
	}
}

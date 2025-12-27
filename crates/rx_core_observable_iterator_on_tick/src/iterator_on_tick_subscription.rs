use std::{iter::Peekable, marker::PhantomData};

use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Never, Observer, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension, SharedSubscriber,
	Signal, Subscriber, SubscriptionLike, WorkCancellationId, WorkResult,
};

use crate::observable::OnTickObservableOptions;

struct OnTickIteratorState<Destination, Iterator>
where
	Destination: Subscriber<In = Iterator::Item, InError = Never>,
	Iterator: IntoIterator,
	Iterator::Item: Signal,
{
	observed_ticks: usize,
	emit_at_every_nth_tick: usize,
	destination: SharedSubscriber<Destination>,
	peekable_iterator: Peekable<Iterator::IntoIter>,
}

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
pub struct OnTickIteratorSubscription<Destination, Iterator, S>
where
	Destination: Subscriber<In = Iterator::Item, InError = Never>,
	Iterator: IntoIterator,
	Iterator::Item: Signal,
	S: Scheduler,
{
	scheduler: SchedulerHandle<S>,
	#[destination]
	destination: SharedSubscriber<Destination>,
	owner_id: Option<WorkCancellationId>,
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
		options: OnTickObservableOptions,
		scheduler: SchedulerHandle<S>,
	) -> Self {
		let mut scheduler_clone = scheduler.clone();
		let mut peekable_iterator = iterator.peekable();
		let mut destination = SharedSubscriber::new(destination);

		if options.emit_at_every_nth_tick == 0 {
			return OnTickIteratorSubscription {
				destination,
				owner_id: None,
				scheduler,
				_phantom_data: PhantomData,
			};
		}

		if options.start_on_subscribe
			&& let Some(next) = peekable_iterator.next()
		{
			destination.next(next);
		}

		let owner_id = {
			let mut scheduler = scheduler_clone.lock();
			let owner_id = scheduler.generate_cancellation_id();

			let mut state = OnTickIteratorState::<Destination, Iterator> {
				observed_ticks: 0,
				emit_at_every_nth_tick: options.emit_at_every_nth_tick,
				peekable_iterator,
				destination: destination.clone(),
			};

			scheduler.schedule_continuous_work(
				move |_, _context| {
					state.observed_ticks += 1;

					let mut destination = state.destination.lock();
					if destination.is_closed() {
						return WorkResult::Done;
					}

					if state
						.observed_ticks
						.is_multiple_of(state.emit_at_every_nth_tick)
						&& let Some(value) = state.peekable_iterator.next()
					{
						state.observed_ticks = 0;
						destination.next(value);
						if state.peekable_iterator.peek().is_none() {
							destination.complete();
							destination.unsubscribe();
							WorkResult::Done
						} else {
							WorkResult::Pending
						}
					} else {
						WorkResult::Pending
					}
				},
				owner_id,
			);

			owner_id
		};

		OnTickIteratorSubscription {
			destination,
			scheduler,
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
			self.scheduler.lock().cancel(owner_id);
		}
		self.destination.unsubscribe();
	}
}

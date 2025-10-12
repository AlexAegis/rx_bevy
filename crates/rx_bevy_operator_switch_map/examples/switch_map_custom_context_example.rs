use rx_bevy::{
	context::{DropUnsafeSubscriptionContext, SubscriptionContext},
	heap_allocator_context::{
		ErasedSubscriberHeapAllocator, ScheduledSubscriptionHeapAllocator, SubscriberHeapAllocator,
		UnscheduledSubscriptionHeapAllocator,
	},
	prelude::*,
};

struct CustomContext;

impl SubscriptionContext for CustomContext {
	type DestinationAllocator<Destination>
		= SubscriberHeapAllocator<Self>
	where
		Destination: 'static + Subscriber<Context = Self> + Send + Sync;

	type ErasedDestinationAllocator<In, InError>
		= ErasedSubscriberHeapAllocator<Self>
	where
		In: SignalBound,
		InError: SignalBound;

	type ScheduledSubscriptionAllocator<Subscription>
		= ScheduledSubscriptionHeapAllocator<Self>
	where
		Subscription: 'static + ObservableSubscription<Context = Self> + Send + Sync;

	type UnscheduledSubscriptionAllocator<Subscription>
		= UnscheduledSubscriptionHeapAllocator<Self>
	where
		Subscription: 'static + SubscriptionLike<Context = Self> + Send + Sync;

	type DropSafety = DropUnsafeSubscriptionContext;

	fn create_context_to_unsubscribe_on_drop() -> Self {
		panic!("Don't worry about me");
	}
}

/// Since all subscriptions present here are inert, it's safe to use an drop-unsafe context
fn main() {
	let mut context = CustomContext;

	let mut subscription = (1..=3)
		.into_observable::<CustomContext>()
		.finalize(|_context| println!("finalize: upstream"))
		.tap_next(|n, _context| println!("emit (source): {n}"))
		.switch_map(|next| {
			IteratorObservable::new(next..=3)
				.map(move |i| format!("from {next} through 3, current: {i}"))
				.finalize(|_context| println!("finalize: inner")) // TODO: Fix, this finalizer isn't running
				.tap_next(|n, _context| println!("emit (inner): '{n}'"))
		})
		.finalize(|_context| println!("finalize: downstream"))
		.subscribe(PrintObserver::new("switch_map"), &mut context);
	subscription.unsubscribe(&mut context);
}

use disqualified::ShortName;
use rx_core_traits::{
	Never, ObservableOutput, Observer, ObserverInput, ObserverUpgradesToSelf,
	PrimaryCategorySubscriber, Subscriber, SubscriptionContext, SubscriptionLike, Teardown,
	TeardownCollection, Tick, Tickable, WithPrimaryCategory, WithSubscriptionContext,
};

#[derive(Debug)]
pub struct ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	destination: Destination,
}

impl<Destination> ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}
}

impl<Destination> ObservableOutput for ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	type Out = Destination::In;
	type OutError = Never;
}

impl<Destination> ObserverInput for ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	type In = Destination::In;
	type InError = Never;
}

impl<Destination> WithSubscriptionContext for ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	type Context = Destination::Context;
}

impl<Destination> WithPrimaryCategory for ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<Destination> ObserverUpgradesToSelf for ErrorBoundarySubscriber<Destination> where
	Destination: Subscriber<InError = Never>
{
}

impl<Destination> Observer for ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error(
		&mut self,
		_error: Self::InError,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		// The operator only compiles if the upstream error is of type `Never`,
		// which is impossible to construct as it's an enum with no variants.

		// It'd be an interesting miracle if this panic was ever triggered.
		unreachable!(
			"An error was observed by {}, but it shouldn't have as `Never` is not a constructable type.",
			ShortName::of::<Self>()
		)
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}

impl<Destination> Tickable for ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	#[inline]
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.tick(tick, context);
	}
}

impl<Destination> SubscriptionLike for ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.unsubscribe(context);
	}
}

impl<Destination> TeardownCollection for ErrorBoundarySubscriber<Destination>
where
	Destination: Subscriber<InError = Never>,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

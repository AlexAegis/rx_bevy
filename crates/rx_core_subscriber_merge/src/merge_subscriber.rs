use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_rc::RcSubscriber;
use rx_core_traits::{
	Observable, Observer, Signal, Subscriber, SubscriptionClosedFlag, SubscriptionContext,
	SubscriptionLike, Teardown, TeardownCollection, Tick, Tickable,
};

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
#[derive(RxSubscriber)]
#[rx_in(InnerObservable)]
#[rx_in_error(InnerObservable::OutError)]
#[rx_context(Destination::Context)]
pub struct MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	pub(crate) destination: RcSubscriber<Destination>,
	pub(crate) inner_subscriptions:
		Vec<<InnerObservable as Observable>::Subscription<RcSubscriber<Destination>>>,
	pub(crate) closed_flag: SubscriptionClosedFlag,
}

impl<InnerObservable, Destination> MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	pub fn new(
		destination: Destination,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			destination: RcSubscriber::new(destination, context),
			inner_subscriptions: Vec::new(),
			closed_flag: false.into(),
		}
	}

	#[inline]
	fn unsubscribe_all_inner(
		&mut self,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		for mut inner_subscription in self.inner_subscriptions.drain(..) {
			inner_subscription.unsubscribe(context);
		}
	}
}

impl<InnerObservable, Destination> Observer for MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	fn next(
		&mut self,
		mut next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			let subscription = next.subscribe(self.destination.clone(), context);

			self.inner_subscriptions.push(subscription);
		}
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.unsubscribe_all_inner(context);
			self.destination.error(error, context);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.destination.complete(context);
		}
	}
}

impl<InnerObservable, Destination> Tickable for MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		// The RcSubscriber ensures only one tick reaches downstream, while
		// still ticking all inner subscribtions
		for inner_subscription in self.inner_subscriptions.iter_mut() {
			inner_subscription.tick(tick, context);
		}

		if self.inner_subscriptions.is_empty() {
			// The inner observable will tick downstream, only directly tick downstream if there is no inner
			self.destination.tick(tick, context);
		}
	}
}

impl<InnerObservable, Destination> SubscriptionLike
	for MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed_flag.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		// An upstream unsubscribe stops everything!
		if !self.is_closed() {
			self.closed_flag.close();
			self.unsubscribe_all_inner(context);
			self.destination.unsubscribe(context);
		}
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.destination.add_downstream_teardown(teardown, context);
		} else {
			teardown.execute(context);
		}
	}
}

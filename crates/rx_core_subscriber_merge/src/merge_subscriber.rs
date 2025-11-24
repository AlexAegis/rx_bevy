use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_rc::RcSubscriber;
use rx_core_traits::{
	Observable, Observer, Subscriber, SubscriptionClosedFlag, SubscriptionContext,
	SubscriptionLike, Teardown, TeardownCollection, Tick, Tickable,
};

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
#[derive(RxSubscriber)]
#[rx_in(InnerObservable)]
#[rx_in_error(InnerObservable::OutError)]
#[rx_context(Destination::Context)]
pub struct MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
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
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
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
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
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
			//self.destination.access_with_context_mut(
			//	|inner, _context| {
			//		inner.inner_is_complete = false;
			//		inner.outer_is_complete = false;
			//	},
			//	context,
			//);

			let subscription =
				next.subscribe(self.destination.clone_with_context(context), context);

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
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
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
		// TODO: Since there's multiple things to tick here, and they all go downstream, this will cause problems, and they must be filtered as they join back
		for inner_subscription in self.inner_subscriptions.iter_mut() {
			inner_subscription.tick(tick.clone(), context);
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
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
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

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		// An upstream unsubscribe stops everything!
		if !self.is_closed() {
			self.closed_flag.close();

			self.unsubscribe_all_inner(context);
			self.destination.unsubscribe(context);
			//self.destination.access_with_context_mut(
			//	|inner, context| {
			//		inner.downstream_destination.unsubscribe(context);
			//	},
			//	context,
			//);
		}
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
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
			let mut teardown = Some(teardown);
			self.destination.access_with_context_mut(
				|inner, context| {
					let teardown = teardown.take().unwrap();
					inner.add_teardown(teardown, context);
				},
				context,
			);
		} else {
			teardown.execute(context);
		}
	}
}

impl<InnerObservable, Destination> Drop for MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	#[inline]
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = InnerObservable::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}

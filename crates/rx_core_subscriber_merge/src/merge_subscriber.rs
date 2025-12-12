use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_higher_order::{
	HigherOrderSubscriberFactory, HigherOrderSubscriberProvider,
};
use rx_core_subscriber_rc::RcSubscriber;
use rx_core_traits::{
	Observable, Observer, Signal, Subscriber, SubscriptionClosedFlag, SubscriptionLike, Teardown,
	TeardownCollection,
};

pub struct MergeSubscriberProvider;

impl HigherOrderSubscriberProvider for MergeSubscriberProvider {
	type HigherOrderSubscriber<InnerObservable, Destination>
		= MergeSubscriber<InnerObservable, Destination>
	where
		InnerObservable: Observable + Signal,
		Destination:
			'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>;
}

impl<InnerObservable, Destination> HigherOrderSubscriberFactory<Destination>
	for MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn new_from_destination(destination: Destination) -> Self {
		Self::new(destination)
	}
}

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
#[derive(RxSubscriber)]
#[rx_in(InnerObservable)]
#[rx_in_error(InnerObservable::OutError)]
pub struct MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub(crate) destination: RcSubscriber<Destination>,
	pub(crate) inner_subscriptions:
		Vec<<InnerObservable as Observable>::Subscription<RcSubscriber<Destination>>>,
	pub(crate) closed_flag: SubscriptionClosedFlag,
}

impl<InnerObservable, Destination> MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: RcSubscriber::new(destination),
			inner_subscriptions: Vec::new(),
			closed_flag: false.into(),
		}
	}

	#[inline]
	fn unsubscribe_all_inner(&mut self) {
		for mut inner_subscription in self.inner_subscriptions.drain(..) {
			inner_subscription.unsubscribe();
		}
	}
}

impl<InnerObservable, Destination> Observer for MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn next(&mut self, mut next: Self::In) {
		if !self.is_closed() {
			let subscription = next.subscribe(self.destination.clone());

			self.inner_subscriptions.push(subscription);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.unsubscribe_all_inner();
			self.destination.error(error);
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			self.destination.complete();
		}
	}
}

impl<InnerObservable, Destination> SubscriptionLike
	for MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed_flag.is_closed()
	}

	fn unsubscribe(&mut self) {
		// An upstream unsubscribe stops everything!
		if !self.is_closed() {
			self.closed_flag.close();
			self.unsubscribe_all_inner();
			self.destination.unsubscribe();
		}
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for MergeSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		if !self.is_closed() {
			self.destination.add_downstream_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_higher_order::{
	HigherOrderSubscriberFactory, HigherOrderSubscriberProvider,
};
use rx_core_subscriber_rc::RcSubscriber;
use rx_core_traits::{
	Observable, Observer, Signal, Subscriber, SubscriptionClosedFlag, SubscriptionLike, Teardown,
	TeardownCollection,
};

pub struct SwitchSubscriberProvider;

impl HigherOrderSubscriberProvider for SwitchSubscriberProvider {
	type HigherOrderSubscriber<InnerObservable, Destination>
		= SwitchSubscriber<InnerObservable, Destination>
	where
		InnerObservable: Observable + Signal,
		Destination:
			'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>;
}

impl<InnerObservable, Destination> HigherOrderSubscriberFactory<Destination>
	for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn new_from_destination(destination: Destination, _concurrency_limit: usize) -> Self {
		Self::new(destination)
	}
}

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
#[derive(RxSubscriber)]
#[rx_in(InnerObservable)]
#[rx_in_error(InnerObservable::OutError)]
pub struct SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	destination: RcSubscriber<Destination>,
	inner_subscription:
		Option<<InnerObservable as Observable>::Subscription<RcSubscriber<Destination>>>,
	closed_flag: SubscriptionClosedFlag,
}

impl<InnerObservable, Destination> SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: RcSubscriber::new(destination),
			inner_subscription: None,
			closed_flag: false.into(),
		}
	}

	#[inline]
	fn unsubscribe_inner(&mut self) {
		if let Some(mut inner_subscription) = self.inner_subscription.take() {
			inner_subscription.unsubscribe();
		}
	}
}

impl<InnerObservable, Destination> Observer for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn next(&mut self, mut next: Self::In) {
		if !self.is_closed() {
			self.unsubscribe_inner();

			let subscription = next.subscribe(self.destination.clone());

			self.inner_subscription = Some(subscription);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.unsubscribe_inner();
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
	for SwitchSubscriber<InnerObservable, Destination>
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

			self.unsubscribe_inner();
			self.destination.unsubscribe();
		}
	}
}

impl<InnerObservable, Destination> TeardownCollection
	for SwitchSubscriber<InnerObservable, Destination>
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

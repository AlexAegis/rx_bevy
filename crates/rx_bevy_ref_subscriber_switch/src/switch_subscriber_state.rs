use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use rx_bevy_core::{
	Observable, Observer, SharedDestination, SharedSubscriber, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Tick,
};

use rx_bevy_subscriber_detached::DetachedSubscriber;

pub struct SwitchSubscriberState<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static + SharedDestination<Access = Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Sharer: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	pub(crate) destination: SharedSubscriber<Destination, Sharer>,
	pub(crate) inner_subscription: Option<<InnerObservable as Observable>::Subscription>,
	pub(crate) closed: bool,
	pub(crate) is_complete: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination, Sharer>
	SwitchSubscriberState<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static + SharedDestination<Access = Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Sharer: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: SharedSubscriber::new(destination),
			inner_subscription: None,
			closed: false,
			is_complete: false,
			_phantom_data: PhantomData,
		}
	}

	pub(crate) fn unsubscribe_inner_subscription(
		&mut self,
		context: &mut <InnerObservable::Subscription as SignalContext>::Context,
	) {
		if let Some(mut inner_subscription) = self.inner_subscription.take() {
			inner_subscription.unsubscribe(context);
		}
	}

	pub(crate) fn create_next_subscription(
		&mut self,
		mut next: InnerObservable,
		state_ref: Rc<RefCell<Self>>,
		context: &mut <InnerObservable::Subscription as SignalContext>::Context,
	) {
		let mut subscription =
			next.subscribe(DetachedSubscriber::new(self.destination.clone()), context);

		if subscription.is_closed() {
			self.complete_if_can(context);
		} else {
			// If it's already closed, this would run immediately panic on the
			// fact that this function is running with the RefCell already borrowed
			subscription.add_fn(
				move |context| {
					state_ref.borrow_mut().complete_if_can(context);
				},
				context,
			);
		}

		self.inner_subscription = Some(subscription);
	}

	pub(crate) fn complete_if_can(
		&mut self,
		context: &mut <InnerObservable::Subscription as SignalContext>::Context,
	) {
		if self.is_complete && self.inner_subscription.is_none() {
			self.destination.complete(context);
			self.unsubscribe(context);
		}
	}

	pub(crate) fn error(
		&mut self,
		error: InnerObservable::OutError,
		context: &mut <InnerObservable::Subscription as SignalContext>::Context,
	) {
		self.destination.error(error, context);
		self.unsubscribe(context);
	}

	pub(crate) fn tick(
		&mut self,
		tick: Tick,
		context: &mut <InnerObservable::Subscription as SignalContext>::Context,
	) {
		self.destination.tick(tick, context);
	}

	pub(crate) fn unsubscribe(
		&mut self,
		context: &mut <InnerObservable::Subscription as SignalContext>::Context,
	) {
		self.closed = true;
		if let Some(mut inner_subscription) = self.inner_subscription.take() {
			inner_subscription.unsubscribe(context);
		}
		self.destination.unsubscribe(context);
	}
}

impl<InnerObservable, Destination, Sharer> Drop
	for SwitchSubscriberState<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static + SharedDestination<Access = Destination>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Sharer: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn drop(&mut self) {
		if !self.closed {
			let mut context = self.destination.get_unsubscribe_context();
			self.unsubscribe(&mut context);
		}
	}
}

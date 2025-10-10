use short_type_name::short_type_name;

use crate::{
	NotifiableSubscription, SignalContext, SubscriptionLike, SubscriptionNotification, Teardown,
	Tick, Tickable, WithContext,
};
use std::fmt::Debug;

/// The base subscription implementation commonly used by other subscription
/// implementations.
///
/// This struct is just a collection of teardown closures, stored as the
/// closure itself.
///
/// This collection of closures represent the resources held by the
/// subscription. To release the resources the subscription must be unsubscribed
/// upon which the collection is drained, and the closures are called,
/// effectively dropping everything held by the subscription before the
/// subscription itself is dropped.
pub struct SubscriptionData<Context>
where
	Context: SignalContext,
{
	is_closed: bool,
	/// Must be stored as function reference or else Context would be forced to
	/// also be 'static when we want to use this as a `dyn SubscriptionLike`
	/// trait object, due to variance as the accepting functions signature is
	/// `impl SubscriptionLike<Context = Context> + 'static`
	notifiable_subscriptions: Vec<Box<dyn FnMut(SubscriptionNotification<Context>, &mut Context)>>,
	finalizers: Vec<Box<dyn FnOnce(&mut Context)>>,
}

impl<Context> SubscriptionData<Context>
where
	Context: SignalContext,
{
	pub fn new_from_resource(subscription: NotifiableSubscription<Context>) -> Self {
		let is_closed = subscription.is_closed();
		let notifiable_subscriptions = if let Some(notifiable_subscription) = subscription.take() {
			vec![notifiable_subscription]
		} else {
			Vec::new()
		};

		Self {
			is_closed,
			notifiable_subscriptions,
			finalizers: Vec::new(),
		}
	}

	pub fn add_notifiable(
		&mut self,
		subscription: NotifiableSubscription<Context>,
		context: &mut Context,
	) {
		if let Some(mut notifiable_subscription) = subscription.take() {
			if self.is_closed() {
				(notifiable_subscription)(SubscriptionNotification::Unsubscribe, context)
			}
			self.notifiable_subscriptions.push(notifiable_subscription);
		}
	}
}

impl<Context> Default for SubscriptionData<Context>
where
	Context: SignalContext,
{
	fn default() -> Self {
		Self {
			notifiable_subscriptions: Vec::new(),
			finalizers: Vec::new(),
			is_closed: false,
		}
	}
}

impl<Context> WithContext for SubscriptionData<Context>
where
	Context: SignalContext,
{
	type Context = Context;
}

impl<Context> Tickable for SubscriptionData<Context>
where
	Context: SignalContext,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		for notifiable_subscription in self.notifiable_subscriptions.iter_mut() {
			(notifiable_subscription)(SubscriptionNotification::Tick(tick.clone()), context);
		}
	}
}

impl<Context> SubscriptionLike for SubscriptionData<Context>
where
	Context: SignalContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		if !self.is_closed() {
			self.is_closed = true;

			for mut notifiable_subscription in self.notifiable_subscriptions.drain(..) {
				(notifiable_subscription)(SubscriptionNotification::Unsubscribe, context);
			}

			for teardown in self.finalizers.drain(..) {
				(teardown)(context);
			}
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		if self.is_closed() {
			// If this subscription is already closed, the newly added teardown
			// is immediately executed.
			teardown.execute(context);
		} else if let Some(teardown_fn) = teardown.take() {
			self.finalizers.push(teardown_fn);
		}
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		// May or may not panic, depending on the context used.
		// If you want to make sure it doesn't panic, use DropSafe contexts!
		// If you do need to use DropUnsafe contexts, make sure you unsubscribe
		// it before letting it go out of scope and drop!
		Context::create_context_to_unsubscribe_on_drop()
	}
}

impl<Context> Debug for SubscriptionData<Context>
where
	Context: SignalContext,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"{} {{ is_closed: {}, finalizers: {} }}",
			short_type_name::<Self>(),
			self.is_closed(),
			self.finalizers.len()
		))
	}
}

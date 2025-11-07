use disqualified::ShortName;

use crate::{
	NotifiableSubscription, SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike,
	SubscriptionNotification, Teardown, Tick, Tickable, WithSubscriptionContext,
};
use std::{fmt::Debug, vec};

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
	Context: SubscriptionContext,
{
	closed_flag: SubscriptionClosedFlag,
	/// Must be stored as function reference or else Context would be forced to
	/// also be 'static when we want to use this as a `dyn SubscriptionLike`
	/// trait object, due to variance as the accepting functions signature is
	/// `impl SubscriptionLike<Context = Context> + 'static`
	notifiable_subscriptions: Vec<
		Box<dyn FnMut(SubscriptionNotification<Context>, &mut Context::Item<'_, '_>) + Send + Sync>,
	>,
	finalizers: Vec<Box<dyn FnOnce(&mut Context::Item<'_, '_>) + Send + Sync>>,
}

impl<Context> SubscriptionData<Context>
where
	Context: SubscriptionContext,
{
	pub fn new_from_resource(subscription: NotifiableSubscription<Context>) -> Self {
		let is_closed = subscription.is_closed();
		let notifiable_subscriptions = if let Some(notifiable_subscription) = subscription.take() {
			vec![notifiable_subscription]
		} else {
			Vec::new()
		};

		Self {
			closed_flag: is_closed.into(),
			notifiable_subscriptions,
			finalizers: Vec::new(),
		}
	}

	pub fn add_notifiable(
		&mut self,
		subscription: NotifiableSubscription<Context>,
		context: &mut Context::Item<'_, '_>,
	) {
		if let Some(mut notifiable_subscription) = subscription.take() {
			if self.is_closed() {
				(notifiable_subscription)(SubscriptionNotification::Unsubscribe, context)
			} else {
				self.notifiable_subscriptions.push(notifiable_subscription);
			}
		}
	}

	pub fn new_with_teardown(teardown: Teardown<Context>) -> Self {
		if let Some(teardown) = teardown.take() {
			Self {
				closed_flag: false.into(),
				finalizers: vec![teardown],
				notifiable_subscriptions: Vec::new(),
			}
		} else {
			Self::default()
		}
	}
}

impl<Context> Default for SubscriptionData<Context>
where
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			notifiable_subscriptions: Vec::new(),
			finalizers: Vec::new(),
			closed_flag: false.into(),
		}
	}
}

impl<Context> WithSubscriptionContext for SubscriptionData<Context>
where
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Context> Tickable for SubscriptionData<Context>
where
	Context: SubscriptionContext,
{
	#[track_caller]
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		for notifiable_subscription in self.notifiable_subscriptions.iter_mut() {
			(notifiable_subscription)(SubscriptionNotification::Tick(tick.clone()), context);
		}
	}
}

impl<Context> SubscriptionLike for SubscriptionData<Context>
where
	Context: SubscriptionContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut Context::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();

			for mut notifiable_subscription in self.notifiable_subscriptions.drain(..) {
				(notifiable_subscription)(SubscriptionNotification::Unsubscribe, context);
			}

			for teardown in self.finalizers.drain(..) {
				(teardown)(context);
			}
		}
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if self.is_closed() {
			// If this subscription is already closed, the newly added teardown
			// is immediately executed.
			teardown.execute(context);
		} else if let Some(teardown_fn) = teardown.take() {
			self.finalizers.push(teardown_fn);
		}
	}
}

impl<Context> Debug for SubscriptionData<Context>
where
	Context: SubscriptionContext,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"{} {{ is_closed: {}, finalizers: {} }}",
			ShortName::of::<Self>(),
			self.is_closed(),
			self.finalizers.len()
		))
	}
}

impl<Context> Drop for SubscriptionData<Context>
where
	Context: SubscriptionContext,
{
	fn drop(&mut self) {
		//  && self.has_something_to_unsubscribe() using this is an anti pattern as problems would still appear with `finalize`
		if !self.is_closed() {
			// TODO: Now the problem is here with subjects
			let mut context = Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}

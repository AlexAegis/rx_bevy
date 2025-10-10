use crate::{
	SignalContext, SubscriptionLike, SubscriptionNotification, TickableSubscription, WithContext,
};

/// A teardown is a closure which owns resources, by the nature of them being
/// moved into said closure. The closure itself is responsible for releasing
/// these resources.
///
/// For example if this resource was a subscription, the closure looks like this:
///
/// ```rs
/// move |context| subscription.unsubscribe(context)
/// ```
///
/// Just like subscriptions, a teardown once closed cannot be opened again.
///
/// [TickableResource] intentionally does not implement [SubscriptionLike] to facilitate
/// the [SubscriptionCollection][crate::SubscriptionCollection] trait which
/// uses [TickableResource] as the base type of operation. Allowing generic functions
/// where you can add anything that is `Into<TickableResource>` such as Subscriptions.
pub struct NotifiableSubscription<Context>
where
	Context: SignalContext,
{
	notify_fn: Option<Box<dyn FnMut(SubscriptionNotification<Context>, &mut Context)>>,
}

impl<Context> NotifiableSubscription<Context>
where
	Context: SignalContext,
{
	pub fn new<F>(f: F) -> Self
	where
		F: 'static + FnMut(SubscriptionNotification<Context>, &mut Context),
	{
		Self {
			notify_fn: Some(Box::new(f)),
		}
	}

	pub fn new_from_box(
		f: Box<dyn FnMut(SubscriptionNotification<Context>, &mut Context)>,
	) -> Self {
		Self { notify_fn: Some(f) }
	}

	/// Consumes the [TickableResource] without executing it, returning the stored
	/// function if it wasn't already closed.
	/// Used when the stored function is moved to somewhere else, like into a
	/// subscription.
	///
	/// It's private to ensure that it's not taken without either executing it
	/// or placing it somewhere else where execution is also guaranteed.
	#[inline]
	pub(crate) fn take(
		mut self,
	) -> Option<Box<dyn FnMut(SubscriptionNotification<Context>, &mut Context)>> {
		self.notify_fn.take()
	}

	/// Immediately consumes and calls the teardowns closure, leaving a None
	/// behind, rendering the teardown permamently closed.
	#[inline]
	pub fn execute(&mut self, action: SubscriptionNotification<Context>, context: &mut Context) {
		if let Some(teardown) = &mut self.notify_fn {
			(teardown)(action, context);
		}
	}

	#[inline]
	pub fn is_closed(&self) -> bool {
		self.notify_fn.is_none()
	}
}

impl<Context> WithContext for NotifiableSubscription<Context>
where
	Context: SignalContext,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for NotifiableSubscription<Context>
where
	Context: SignalContext,
{
	fn is_closed(&self) -> bool {
		self.notify_fn.is_none()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.execute(SubscriptionNotification::Unsubscribe, context);
	}

	fn add_teardown(
		&mut self,
		teardown: super::Teardown<Self::Context>,
		context: &mut Self::Context,
	) {
		self.execute(SubscriptionNotification::Add(teardown), context);
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Self::Context::create_context_to_unsubscribe_on_drop()
	}
}

impl<Context> Default for NotifiableSubscription<Context>
where
	Context: SignalContext,
{
	fn default() -> Self {
		Self { notify_fn: None }
	}
}

/// Exposes and respects the original subscriptions closed-ness by storing it
/// in an option.
///
/// This means that when you convert an already closed subscription into a
/// teardown, it will be immediately dropped.
impl<S> From<S> for NotifiableSubscription<S::Context>
where
	S: 'static + TickableSubscription,
{
	fn from(mut subscription: S) -> Self {
		Self {
			notify_fn: if subscription.is_closed() {
				None
			} else {
				Some(Box::new(move |action, context| match action {
					SubscriptionNotification::Tick(tick) => {
						subscription.tick(tick, context);
					}
					SubscriptionNotification::Unsubscribe => {
						subscription.unsubscribe(context);
					}
					SubscriptionNotification::Add(teardown) => {
						subscription.add_teardown(teardown, context);
					}
				}))
			},
		}
	}
}

use crate::{SignalContext, SubscriptionCollection, SubscriptionLike, Teardown};

pub struct InnerSubscription<Context> {
	is_closed: bool,
	finalizers: Vec<Box<dyn FnOnce(&mut Context)>>,
	// Force invariance, the compiler resolves Context to be a bivariant // And now it doesn't report it as bivariant anymore??
	//_phantom_data: PhantomData<*mut (Context)>,
}

impl<Context> InnerSubscription<Context> {
	pub fn new<S, T>(subscription: T) -> Self
	where
		S: SubscriptionLike<Context = Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		let teardown: Teardown<S, S::Context> = subscription.into();

		if let Some(teardown_fn) = teardown.take() {
			Self {
				is_closed: false,
				finalizers: vec![teardown_fn],
				//	_phantom_data: PhantomData,
			}
		} else {
			Self::default()
		}
	}

	pub fn new_fn<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut Context),
	{
		Self::new(Teardown::<Self, Context>::new(f))
	}
}

impl<Context> Default for InnerSubscription<Context> {
	fn default() -> Self {
		Self {
			finalizers: Vec::new(),
			is_closed: false,
			// _phantom_data: PhantomData,
		}
	}
}

impl<Context> SignalContext for InnerSubscription<Context> {
	type Context = Context;
}

impl<Context> SubscriptionLike for InnerSubscription<Context> {
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		if !self.is_closed {
			self.is_closed = true;

			for teardown in self.finalizers.drain(..) {
				(teardown)(context);
			}
		}
	}
}

impl<Context> SubscriptionCollection for InnerSubscription<Context> {
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		let teardown: Teardown<S, S::Context> = subscription.into();
		if self.is_closed() {
			// If this subscription is already closed, the added one is unsubscribed immediately
			teardown.call(context);
		} else {
			if let Some(teardown_fn) = teardown.take() {
				self.finalizers.push(teardown_fn);
			}
		}
	}
}

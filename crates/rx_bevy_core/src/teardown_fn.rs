use crate::{SignalContext, SubscriptionCollection, SubscriptionLike};

pub struct TeardownFn<Context>(
	Option<Box<dyn FnOnce(&mut <Self as SignalContext>::Context) + 'static>>,
);

impl<Context> TeardownFn<Context> {
	pub fn new<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut <Self as SignalContext>::Context),
	{
		Self(Some(Box::new(f)))
	}
}

impl<F, Context> From<F> for TeardownFn<Context>
where
	F: 'static + FnOnce(&mut <Self as SignalContext>::Context),
{
	fn from(teardown: F) -> Self {
		Self(Some(Box::new(teardown)))
	}
}

impl<Context> SignalContext for TeardownFn<Context> {
	type Context = Context;
}

impl<Context> SubscriptionLike for TeardownFn<Context> {
	fn is_closed(&self) -> bool {
		self.0.is_none()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if let Some(teardown_fn) = self.0.take() {
			(teardown_fn)(context)
		}
	}
}

pub trait SubscriptionCollectionTeardownFnExtension: SubscriptionCollection {
	fn add_fn<F>(&mut self, f: F, context: &mut Self::Context)
	where
		F: FnOnce(&mut Self::Context) + 'static,
		TeardownFn<Self::Context>: 'static,
	{
		let teardown: TeardownFn<Self::Context> = f.into();
		self.add(teardown, context)
	}
}

impl<T> SubscriptionCollectionTeardownFnExtension for T where T: SubscriptionCollection {}

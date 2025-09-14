use std::marker::PhantomData;

use crate::SubscriptionLike;

pub struct Teardown<S, Context> {
	teardown_fn: Box<dyn FnOnce(&mut Context)>,
	_phantom_data: PhantomData<S>,
}

impl<S, Context> Teardown<S, Context> {
	pub fn new<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut Context),
	{
		Self {
			teardown_fn: Box::new(f),
			_phantom_data: PhantomData,
		}
	}

	pub fn take(self) -> Box<dyn FnOnce(&mut Context)> {
		self.teardown_fn
	}
}

impl<S> From<S> for Teardown<S, S::Context>
where
	S: 'static + SubscriptionLike,
{
	fn from(mut value: S) -> Self {
		Self {
			teardown_fn: Box::new(move |context| value.unsubscribe(context)),
			_phantom_data: PhantomData,
		}
	}
}

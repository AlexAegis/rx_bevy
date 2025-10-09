use std::{fmt::Debug, marker::PhantomData};

use rx_bevy_core::{
	Observer, ObserverInput, SignalContext, Subscription, SubscriptionLike, Teardown, WithContext,
};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct PrintObserver<In, InError = (), Context = ()>
where
	In: Debug,
	InError: Debug,
	Context: SignalContext,
{
	prefix: Option<&'static str>,
	teardown: Subscription<Context>,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> PrintObserver<In, InError, Context>
where
	In: Debug,
	InError: Debug,
	Context: SignalContext,
{
	pub fn new(message: &'static str) -> Self {
		Self {
			prefix: Some(message),
			teardown: Subscription::default(),
			_phantom_data: PhantomData,
		}
	}

	fn get_prefix(&self) -> String {
		self.prefix
			.map(|prefix| format!("{prefix} - "))
			.unwrap_or_default()
	}
}

impl<In, InError, Context> Default for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
	Context: SignalContext,
{
	fn default() -> Self {
		Self {
			prefix: None,
			teardown: Subscription::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> ObserverInput for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
	Context: SignalContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> Observer for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
	Context: SignalContext,
{
	#[inline]
	fn next(&mut self, next: Self::In, _context: &mut Self::Context) {
		println!("{}next: {:?}", self.get_prefix(), next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		println!("{}error: {:?}", self.get_prefix(), error);
		self.teardown.unsubscribe(context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		println!("{}completed", self.get_prefix());
		self.teardown.unsubscribe(context);
	}

	#[inline]
	fn tick(&mut self, tick: rx_bevy_core::Tick, _context: &mut Self::Context) {
		println!("{}tick: {:?}", self.get_prefix(), tick);
	}
}

impl<In, InError, Context> WithContext for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
	Context: SignalContext,
{
	type Context = Context;
}

impl<In, InError, Context> SubscriptionLike for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
	Context: SignalContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.teardown.is_closed() {
			self.teardown.unsubscribe(context);
			println!("{}unsubscribed", self.get_prefix());
		}
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.teardown.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Context::create_context_to_unsubscribe_on_drop()
	}
}

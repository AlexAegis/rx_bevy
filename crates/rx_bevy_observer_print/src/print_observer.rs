use std::{fmt::Debug, marker::PhantomData};

#[cfg(feature = "tick")]
#[cfg(feature = "channel_context")]
use rx_bevy_core::ChannelContext;
use rx_bevy_core::{
	ExpandableSubscriptionLike, InnerSubscription, Observer, ObserverInput, SubscriptionLike,
	Teardown,
};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct PrintObserver<In, InError = (), Context = ()>
where
	In: Debug,
	InError: Debug,
{
	prefix: Option<&'static str>,

	closed: bool,
	teardown: InnerSubscription,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> PrintObserver<In, InError, Context>
where
	In: Debug,
	InError: Debug,
{
	pub fn new(message: &'static str) -> Self {
		Self {
			prefix: Some(message),
			closed: false,
			teardown: InnerSubscription::new_empty(),
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
{
	fn default() -> Self {
		Self {
			prefix: None,
			closed: false,
			teardown: InnerSubscription::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> ObserverInput for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> Observer for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
{
	type Context = Context;

	#[inline]
	fn next(&mut self, next: Self::In, _context: &mut Self::Context) {
		println!("{}next: {:?}", self.get_prefix(), next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, _context: &mut Self::Context) {
		println!("{}error: {:?}", self.get_prefix(), error);
	}

	#[inline]
	fn complete(&mut self, _context: &mut Self::Context) {
		println!("{}completed", self.get_prefix());
	}

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(&mut self, tick: rx_bevy_core::Tick, _context: &mut Self::Context) {
		println!("{}tick: {:?}", self.get_prefix(), tick);
	}
}

impl<In, InError, Context> SubscriptionLike<Context> for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		if !self.closed {
			self.closed = true;
			self.teardown.unsubscribe(context);
			#[cfg(not(feature = "channel_context"))]
			self.teardown.unsubscribe();
			println!("{}unsubscribed", self.get_prefix());
		}
	}
}

impl<In, InError, Context> ExpandableSubscriptionLike<Context>
	for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
{
	fn add(&mut self, subscription: impl Into<Teardown<Context>>, context: &mut Context) {
		self.teardown.add(subscription, context);
	}
}

use std::{fmt::Debug, marker::PhantomData};

#[cfg(feature = "tick")]
#[cfg(feature = "channel_context")]
use rx_bevy_core::ChannelContext;
use rx_bevy_core::{InnerSubscription, Observer, ObserverInput, SubscriptionLike};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct PrintObserver<In, InError = ()>
where
	In: Debug,
	InError: Debug,
{
	prefix: Option<&'static str>,

	closed: bool,
	teardown: InnerSubscription,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> PrintObserver<In, InError>
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

impl<In, InError> Default for PrintObserver<In, InError>
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

impl<In, InError> ObserverInput for PrintObserver<In, InError>
where
	In: 'static + Debug,
	InError: 'static + Debug,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> Observer for PrintObserver<In, InError>
where
	In: 'static + Debug,
	InError: 'static + Debug,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		#[cfg(feature = "channel_context")] _context: &mut ChannelContext,
	) {
		println!("{}next: {:?}", self.get_prefix(), next);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		#[cfg(feature = "channel_context")] _context: &mut ChannelContext,
	) {
		println!("{}error: {:?}", self.get_prefix(), error);
	}

	#[inline]
	fn complete(&mut self, #[cfg(feature = "channel_context")] _context: &mut ChannelContext) {
		println!("{}completed", self.get_prefix());
	}

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(
		&mut self,
		tick: rx_bevy_core::Tick,
		#[cfg(feature = "channel_context")] _context: &mut ChannelContext,
	) {
		println!("{}tick: {:?}", self.get_prefix(), tick);
	}
}

impl<In, InError> SubscriptionLike for PrintObserver<In, InError>
where
	In: 'static + Debug,
	InError: 'static + Debug,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, #[cfg(feature = "channel_context")] context: &mut ChannelContext) {
		if !self.closed {
			self.closed = true;
			#[cfg(feature = "channel_context")]
			self.teardown.unsubscribe(context);
			#[cfg(not(feature = "channel_context"))]
			self.teardown.unsubscribe();
			println!("{}unsubscribed", self.get_prefix());
		}
	}

	fn add(
		&mut self,
		subscription: Box<dyn SubscriptionLike>,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) {
		#[cfg(feature = "channel_context")]
		self.teardown.add(subscription, context);
		#[cfg(not(feature = "channel_context"))]
		self.teardown.add(subscription);
	}
}

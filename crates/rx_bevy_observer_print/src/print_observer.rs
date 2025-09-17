use std::{fmt::Debug, marker::PhantomData};

use rx_bevy_core::{DropContext, Observer, ObserverInput, SignalContext, SubscriptionLike};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct PrintObserver<In, InError = (), Context = ()>
where
	In: Debug,
	InError: Debug,
{
	prefix: Option<&'static str>,
	closed: bool,
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
	Context: DropContext,
{
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

	#[inline]
	fn tick(&mut self, tick: rx_bevy_core::Tick, _context: &mut Self::Context) {
		println!("{}tick: {:?}", self.get_prefix(), tick);
	}
}

impl<In, InError, Context> SignalContext for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
	Context: DropContext,
{
	type Context = Context;
}

impl<In, InError, Context> SubscriptionLike for PrintObserver<In, InError, Context>
where
	In: 'static + Debug,
	InError: 'static + Debug,
	Context: DropContext,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, _context: &mut Self::Context) {
		if !self.closed {
			self.closed = true;

			println!("{}unsubscribed", self.get_prefix());
		}
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
	}
}

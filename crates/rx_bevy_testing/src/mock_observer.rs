use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, InnerSubscription, Observer, ObserverInput, SignalContext,
	SignalContextDropSafety, SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};
use short_type_name::short_type_name;

#[derive(Debug)]
pub struct MockObserver<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	pub closed: bool,
	teardown: InnerSubscription<MockContext<In, InError, DropSafety>>,
	_phantom_data: PhantomData<(In, InError, DropSafety)>,
}

impl<In, InError, DropSafety> ObserverInput for MockObserver<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	type In = In;
	type InError = InError;
}

#[derive(Debug)]
pub struct MockContext<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	pub values: Vec<In>,
	pub errors: Vec<InError>,
	pub ticks: Vec<Tick>,
	pub completed: usize,
	pub unsubscribes: usize,
	pub adds: usize,
	pub values_after_closed: Vec<In>,
	pub errors_after_closed: Vec<InError>,
	pub ticks_after_closed: Vec<Tick>,
	pub completed_after_closed: usize,
	pub unsubscribes_after_closed: usize,
	pub adds_after_closed: usize,
	_phantom_data: PhantomData<DropSafety>,
}

impl<In, InError, DropSafety> MockContext<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	pub fn nothing_happened_after_closed(&self) -> bool {
		self.values_after_closed.is_empty()
			&& self.errors_after_closed.is_empty()
			&& self.completed_after_closed == 0
			&& self.ticks_after_closed.is_empty()
			&& self.unsubscribes_after_closed == 0
			&& self.adds_after_closed == 0
	}

	pub fn all_values(&self) -> Vec<In>
	where
		In: Clone,
	{
		self.values
			.iter()
			.chain(self.values_after_closed.iter())
			.cloned()
			.collect()
	}

	pub fn all_errors(&self) -> Vec<InError>
	where
		InError: Clone,
	{
		self.errors
			.iter()
			.chain(self.errors_after_closed.iter())
			.cloned()
			.collect()
	}

	pub fn all_completions(&self) -> usize {
		self.completed + self.completed_after_closed
	}

	pub fn all_unsubscribes(&self) -> usize {
		self.unsubscribes + self.unsubscribes_after_closed
	}

	pub fn all_adds(&self) -> usize {
		self.adds + self.adds_after_closed
	}
}

impl<In, InError, DropSafety> DropContext for MockContext<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	/// The DropSafety is parametric for the sake of testability, the context will always panic on drop if not closed to ensure proper tests.
	type DropSafety = DropSafety;

	fn get_context_for_drop() -> Self {
		// While this context could be constructed very easily (It has a
		// [Default] implementation too! This is the reason why this method
		// exists by the way. It just doesn't have the same connotation!)
		// letting subscriptions implicitly unsubscribe on drop would lead to
		// tests that you cannot trust!
		panic!(
			"An unclosed Subscription was dropped during a test! For tests, the context must be explicitly supplied as it stores the data used for asserts! {}",
			short_type_name::<Self>()
		)
	}
}

impl<In, InError, DropSafety> Default for MockContext<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	fn default() -> Self {
		Self {
			values: Vec::with_capacity(1),
			errors: Vec::with_capacity(1),
			ticks: Vec::with_capacity(1),
			values_after_closed: Vec::new(),
			errors_after_closed: Vec::new(),
			ticks_after_closed: Vec::new(),
			completed: 0,
			completed_after_closed: 0,
			unsubscribes: 0,
			unsubscribes_after_closed: 0,
			adds: 0,
			adds_after_closed: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, DropSafety> Observer for MockObserver<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			context.values.push(next);
		} else {
			context.values_after_closed.push(next);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			context.errors.push(error);
			self.unsubscribe(context);
		} else {
			context.errors_after_closed.push(error);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			context.completed += 1;
			self.unsubscribe(context);
		} else {
			context.completed_after_closed += 1;
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			context.ticks.push(tick);
		} else {
			context.ticks_after_closed.push(tick);
		}
	}
}

impl<In, InError, DropSafety> SignalContext for MockObserver<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	type Context = MockContext<In, InError, DropSafety>;
}

impl<In, InError, DropSafety> SubscriptionLike for MockObserver<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.closed = true;
			context.unsubscribes += 1;
		} else {
			context.unsubscribes_after_closed += 1;
		}
		println!(
			"mock context unsub {} {}",
			context.unsubscribes, context.unsubscribes_after_closed
		);
	}

	fn get_unsubscribe_context(&mut self) -> Self::Context {
		MockContext::default()
	}
}

impl<In, InError, DropSafety> Default for MockObserver<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	fn default() -> Self {
		Self {
			closed: false,
			teardown: InnerSubscription::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, DropSafety> SubscriptionCollection for MockObserver<In, InError, DropSafety>
where
	In: 'static,
	InError: 'static,
	DropSafety: SignalContextDropSafety,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		if !self.is_closed() {
			context.adds += 1;
			self.teardown.add(subscription, context);
		} else {
			context.adds_after_closed += 1;
			let teardown: Teardown<S, S::Context> = subscription.into();
			teardown.call(context)
		}
	}
}

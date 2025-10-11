use rx_bevy_core::{
	SubscriptionData, SubscriptionHandle, SubscriptionLike, Tick, Tickable, TickableSubscription,
	WithSubscriptionContext,
};

pub struct CombineLatestSubscription<S1, S2>
where
	S1: TickableSubscription,
	S2: TickableSubscription<Context = S1::Context>,
{
	s1: SubscriptionHandle<S1>,
	s2: SubscriptionHandle<S2>,
	teardown: SubscriptionData<S1::Context>,
}

impl<S1, S2> CombineLatestSubscription<S1, S2>
where
	S1: TickableSubscription,
	S2: TickableSubscription<Context = S1::Context>,
{
	pub fn new(s1: SubscriptionHandle<S1>, s2: SubscriptionHandle<S2>) -> Self {
		Self {
			s1,
			s2,
			teardown: SubscriptionData::default(),
		}
	}
}

impl<S1, S2> Tickable for CombineLatestSubscription<S1, S2>
where
	S1: TickableSubscription,
	S2: TickableSubscription<Context = S1::Context>,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.s1.tick(tick.clone(), context);
		self.s2.tick(tick.clone(), context);
		self.teardown.tick(tick, context);
	}
}

impl<S1, S2> SubscriptionLike for CombineLatestSubscription<S1, S2>
where
	S1: TickableSubscription,
	S2: TickableSubscription<Context = S1::Context>,
{
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.s1.unsubscribe(context);
		self.s2.unsubscribe(context);
		self.teardown.unsubscribe(context);
	}

	fn add_teardown(
		&mut self,
		teardown: rx_bevy_core::Teardown<Self::Context>,
		context: &mut Self::Context,
	) {
		self.teardown.add_teardown(teardown, context);
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.teardown.get_context_to_unsubscribe_on_drop()
	}
}

impl<S1, S2> WithSubscriptionContext for CombineLatestSubscription<S1, S2>
where
	S1: TickableSubscription,
	S2: TickableSubscription<Context = S1::Context>,
{
	type Context = S1::Context;
}

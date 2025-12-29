use rx_core_traits::SubscriberState;

pub trait HigherOrderSubscriberStateConditions {
	fn can_downstream_complete(&self) -> bool;
	fn can_downstream_unsubscribe(&self) -> bool;

	fn on_upstream_error(&mut self);
	fn on_downstream_error(&mut self);
}

impl HigherOrderSubscriberStateConditions for () {
	#[inline]
	fn can_downstream_complete(&self) -> bool {
		true
	}

	#[inline]
	fn can_downstream_unsubscribe(&self) -> bool {
		true
	}

	#[inline]
	fn on_downstream_error(&mut self) {}

	#[inline]
	fn on_upstream_error(&mut self) {}
}

#[derive(Default)]
pub struct HigherOrderSubscriberState<State>
where
	State: HigherOrderSubscriberStateConditions,
{
	pub state: State,
	pub non_completed_subscriptions: usize,
	pub non_unsubscribed_subscriptions: usize,
	pub upstream_subscriber_state: SubscriberState,
	pub downstream_subscriber_state: SubscriberState,
}

impl<State> HigherOrderSubscriberState<State>
where
	State: HigherOrderSubscriberStateConditions,
{
	pub fn new(state: State) -> Self {
		Self {
			state,
			non_completed_subscriptions: 0,
			non_unsubscribed_subscriptions: 0,
			upstream_subscriber_state: SubscriberState::default(),
			downstream_subscriber_state: SubscriberState::default(),
		}
	}

	fn can_downstream_complete(&self) -> bool {
		self.state.can_downstream_complete()
			&& self.non_completed_subscriptions == 0
			&& self.upstream_subscriber_state.is_completed()
			&& !self.downstream_subscriber_state.is_completed()
	}

	fn can_downstream_unsubscribe(&self) -> bool {
		(self.state.can_downstream_unsubscribe()
			&& self.non_unsubscribed_subscriptions == 0
			&& self.upstream_subscriber_state.is_unsubscribed()
			&& !self.downstream_subscriber_state.is_unsubscribed())
			|| self.upstream_subscriber_state.is_errored()
			|| self.downstream_subscriber_state.is_errored()
	}

	pub fn upstream_completed_can_downstream(&mut self) -> bool {
		self.upstream_subscriber_state.complete();
		self.upstream_subscriber_state.unsubscribe();

		self.can_downstream_complete()
	}

	pub fn inner_completed_can_downstream(&mut self) -> bool {
		let downstream_can_complete = self.can_downstream_complete();

		if downstream_can_complete {
			self.downstream_subscriber_state.complete();
		}

		downstream_can_complete
	}

	pub fn inner_unsubscribed_can_downstream(&mut self) -> bool {
		let downstream_can_unsubscribe = self.can_downstream_unsubscribe();

		if downstream_can_unsubscribe {
			self.downstream_subscriber_state
				.unsubscribe_if_not_already();
		}

		downstream_can_unsubscribe
	}

	pub fn upstream_unsubscribe_can_downstream(&mut self) -> bool {
		self.upstream_subscriber_state.unsubscribe_if_not_already();

		self.can_downstream_unsubscribe()
	}

	pub fn upstream_error(&mut self) {
		self.state.on_upstream_error();
		self.upstream_subscriber_state.error();

		if !self.upstream_subscriber_state.is_unsubscribed() {
			self.upstream_subscriber_state.unsubscribe();
		}

		if !self.downstream_subscriber_state.is_unsubscribed() {
			self.downstream_subscriber_state.unsubscribe();
		}
	}

	pub fn downstream_error(&mut self) {
		self.state.on_downstream_error();
		self.downstream_subscriber_state.error();

		if !self.upstream_subscriber_state.is_unsubscribed() {
			self.upstream_subscriber_state.unsubscribe();
		}

		if !self.downstream_subscriber_state.is_unsubscribed() {
			self.downstream_subscriber_state.unsubscribe();
		}
	}
}

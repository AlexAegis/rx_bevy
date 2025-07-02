use rx_bevy_observable::{InnerSubscription, Observer, ObserverInput, SubscriptionLike, Teardown};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct DynFnObserver<In, Error> {
	on_next: Option<Box<dyn FnMut(In)>>,
	on_error: Option<Box<dyn FnMut(Error)>>,
	on_complete: Option<Box<dyn FnOnce()>>,
	on_unsubscribe: Option<Box<dyn FnOnce()>>,

	inner_subscription: InnerSubscription,
}

impl<In, InError> ObserverInput for DynFnObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> Observer for DynFnObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn next(&mut self, next: In) {
		if !self.is_closed() {
			if let Some(on_next) = &mut self.on_next {
				(on_next)(next);
			}
		}
	}

	fn error(&mut self, error: InError) {
		if !self.is_closed() {
			if let Some(on_error) = &mut self.on_error {
				(on_error)(error);
			} else {
				panic!("DynFnObserver without an error observer encountered an error!");
			}

			self.unsubscribe();
		}
	}

	fn complete(&mut self) {
		if !self.is_closed() {
			if let Some(on_complete) = self.on_complete.take() {
				(on_complete)();
			}

			self.unsubscribe();
		}
	}
}

impl<In, InError> SubscriptionLike for DynFnObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn is_closed(&self) -> bool {
		self.inner_subscription.is_closed()
	}

	fn unsubscribe(&mut self) {
		if let Some(on_unsubscribe) = self.on_unsubscribe.take() {
			(on_unsubscribe)();
		}
		self.inner_subscription.unsubscribe();
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.inner_subscription.add(Teardown::Sub(subscription));
	}
}

impl<In, InError> Default for DynFnObserver<In, InError> {
	fn default() -> Self {
		Self {
			on_next: None,
			on_error: None,
			on_complete: None,
			on_unsubscribe: None,
			inner_subscription: InnerSubscription::new_empty(),
		}
	}
}

impl<In, InError> DynFnObserver<In, InError> {
	pub fn with_next<OnPush: 'static + FnMut(In)>(self, next: OnPush) -> Self {
		Self {
			on_next: Some(Box::new(next)),
			..self
		}
	}

	pub fn with_error<OnError: 'static + FnMut(InError)>(self, error: OnError) -> Self {
		Self {
			on_error: Some(Box::new(error)),
			..self
		}
	}

	pub fn with_complete<OnComplete: 'static + FnOnce()>(self, complete: OnComplete) -> Self {
		Self {
			on_complete: Some(Box::new(complete)),
			..self
		}
	}

	pub fn with_unsubscribe<OnUnsubscribe: 'static + FnOnce()>(
		self,
		on_unsubscribe: OnUnsubscribe,
	) -> Self {
		Self {
			on_unsubscribe: Some(Box::new(on_unsubscribe)),
			..self
		}
	}
}

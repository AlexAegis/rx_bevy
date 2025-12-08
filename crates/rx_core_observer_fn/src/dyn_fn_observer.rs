use rx_core_traits::{
	Observer, ObserverInput, ObserverSubscriber, PrimaryCategoryObserver, Signal,
	UpgradeableObserver, WithPrimaryCategory,
};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct DynFnObserver<In, Error> {
	on_next: Option<Box<dyn FnMut(In) + Send + Sync>>,
	on_error: Option<Box<dyn FnMut(Error) + Send + Sync>>,
	on_complete: Option<Box<dyn FnOnce() + Send + Sync>>,
	on_unsubscribe: Option<Box<dyn FnOnce() + Send + Sync>>,
}

impl<In, InError> DynFnObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn with_next<OnNext: 'static + FnMut(In) + Send + Sync>(mut self, on_next: OnNext) -> Self {
		self.on_next.replace(Box::new(on_next));
		self
	}

	pub fn with_error<OnError: 'static + FnMut(InError) + Send + Sync>(
		mut self,
		on_error: OnError,
	) -> Self {
		self.on_error.replace(Box::new(on_error));
		self
	}

	pub fn with_complete<OnComplete: 'static + FnOnce() + Send + Sync>(
		mut self,
		on_complete: OnComplete,
	) -> Self {
		self.on_complete.replace(Box::new(on_complete));
		self
	}

	/// The difference between this and the regular `add` method that the
	/// teardown passed into `add` is always guaranteed to be executed, when
	/// call `add` on an already closed [Subscription], the teardown is
	/// immediately executed to ensure this.
	/// This method however does not need to guarantee that, as it's meant to be
	/// used during the creation of the observer, which enables us to have
	/// a nicer signature by leaving the context argument off from the method,
	/// and making it chainable.
	pub fn with_unsubscribe<OnUnsubscribe: 'static + FnOnce() + Send + Sync>(
		mut self,
		on_unsubscribe: OnUnsubscribe,
	) -> Self {
		self.on_unsubscribe.replace(Box::new(on_unsubscribe));
		self
	}
}

impl<In, InError> ObserverInput for DynFnObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> WithPrimaryCategory for DynFnObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type PrimaryCategory = PrimaryCategoryObserver;
}

impl<In, InError> UpgradeableObserver for DynFnObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Upgraded = ObserverSubscriber<Self>;

	fn upgrade(self) -> Self::Upgraded {
		ObserverSubscriber::new(self)
	}
}

impl<In, InError> Observer for DynFnObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn next(&mut self, next: In) {
		if let Some(on_next) = &mut self.on_next {
			(on_next)(next);
		}
	}

	fn error(&mut self, error: InError) {
		if let Some(on_error) = &mut self.on_error {
			(on_error)(error);
		} else {
			panic!("DynFnObserver without an error observer encountered an error!");
		}
	}

	fn complete(&mut self) {
		if let Some(on_complete) = self.on_complete.take() {
			(on_complete)();
		}
	}
}

impl<In, InError> Default for DynFnObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn default() -> Self {
		Self {
			on_next: None,
			on_error: None,
			on_complete: None,
			on_unsubscribe: None,
		}
	}
}

use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{Observer, Signal};

/// A simple observer that prints out received values using [std::fmt::Debug]
#[derive(RxObserver)]
#[rx_in(In)]
#[rx_in_error(InError)]
pub struct DynFnObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	on_next: Option<Box<dyn FnMut(In) + Send + Sync>>,
	on_error: Option<Box<dyn FnOnce(InError) + Send + Sync>>,
	on_complete: Option<Box<dyn FnOnce() + Send + Sync>>,
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

	pub fn with_error<OnError: 'static + FnOnce(InError) + Send + Sync>(
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
		if let Some(on_error) = self.on_error.take() {
			(on_error)(error);
		} else {
			panic!("DynFnObserver without an error observer encountered an uncaught error!");
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
		}
	}
}

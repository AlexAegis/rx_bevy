use std::{fmt::Debug, marker::PhantomData};

use rx_bevy_observable::{
	Observer, ObserverInput, UpgradeableObserver, prelude::ObserverSubscriber,
};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct PrintObserver<In, InError = ()>
where
	In: Debug,
	InError: Debug,
{
	prefix: Option<&'static str>,
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
			_phantom_data: PhantomData,
		}
	}

	fn get_prefix(&self) -> String {
		self.prefix
			.map(|prefix| format!("{} - ", prefix))
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
	fn next(&mut self, next: Self::In) {
		println!("{}next: {:?}", self.get_prefix(), next);
	}

	fn error(&mut self, error: Self::InError) {
		println!("{}error: {:?}", self.get_prefix(), error);
	}

	fn complete(&mut self) {
		println!("{}completed", self.get_prefix());
	}
}

impl<In, InError> UpgradeableObserver for PrintObserver<In, InError>
where
	In: 'static + Debug,
	InError: 'static + Debug,
{
	type Subscriber = ObserverSubscriber<Self>;

	fn upgrade(self) -> Self::Subscriber {
		ObserverSubscriber::new(self)
	}
}

use crate::{Observer, ObserverInput, Subscriber, SubscriptionLike};

pub enum LazySubscriber<Destination>
where
	Destination: Subscriber,
{
	Empty,
	Initialized(Destination),
}

impl<Destination> Default for LazySubscriber<Destination>
where
	Destination: Subscriber,
{
	fn default() -> Self {
		Self::Empty
	}
}

impl<Destination> LazySubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new() -> Self {
		Self::default()
	}

	pub fn new_initialized(destination: Destination) -> Self {
		Self::Initialized(destination)
	}

	pub fn initialize(&mut self, destination: Destination) -> &mut Self {
		// if it's already initialized, unsubscribe, although this shouldn't happen
		if let LazySubscriber::Initialized(d) = self {
			d.unsubscribe();
		}

		*self = LazySubscriber::Initialized(destination);
		self
	}
}

impl<Destination> ObserverInput for LazySubscriber<Destination>
where
	Destination: Subscriber,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for LazySubscriber<Destination>
where
	Destination: Subscriber,
{
	fn next(&mut self, next: Self::In) {
		match self {
			Self::Initialized(destination) => destination.next(next),
			Self::Empty => panic!("next called on an uninitialized lazy subscriber!"),
		}
	}

	fn error(&mut self, error: Self::InError) {
		match self {
			Self::Initialized(destination) => destination.error(error),
			Self::Empty => panic!("error called on an uninitialized lazy subscriber!"),
		}
	}

	fn complete(&mut self) {
		match self {
			Self::Initialized(destination) => destination.complete(),
			Self::Empty => panic!("complete called on an uninitialized lazy subscriber!"),
		}
	}
}

impl<Destination> SubscriptionLike for LazySubscriber<Destination>
where
	Destination: Subscriber,
{
	fn is_closed(&self) -> bool {
		match self {
			Self::Initialized(destination) => destination.is_closed(),
			Self::Empty => panic!("is_closed called on an uninitialized lazy subscriber!"),
		}
	}

	fn unsubscribe(&mut self) {
		match self {
			Self::Initialized(destination) => destination.unsubscribe(),
			Self::Empty => panic!("unsubscribe called on an uninitialized lazy subscriber!"),
		}
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		match self {
			Self::Initialized(destination) => destination.add(subscription),
			Self::Empty => panic!("add called on an uninitialized lazy subscriber!"),
		}
	}
}

impl<Destination> Drop for LazySubscriber<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		self.unsubscribe()
	}
}

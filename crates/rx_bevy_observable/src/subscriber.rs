use crate::{Observer, Subscription};

/// A Subscriber is both an Observer and a Subscription
/// It owns an observer
pub struct Subscriber<Destination> {
	pub destination: Option<Destination>,
}

impl<In, Destination> Observer for Subscriber<Destination>
where
	Destination: Observer<In = In>,
{
	type In = Destination::In;

	fn on_push(&mut self, next: In) {
		if let Some(ref mut observer) = self.destination {
			observer.on_push(next);
		}
	}
}

impl<Destination> Subscription for Subscriber<Destination> {
	fn is_closed(&self) -> bool {
		self.destination.is_none()
	}

	fn unsubscribe(&mut self) {
		self.destination = None;
	}
}

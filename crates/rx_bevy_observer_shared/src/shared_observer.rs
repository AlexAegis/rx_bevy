use std::sync::{Arc, Mutex};

use rx_bevy_observable::{Observer, ObserverInput};

pub struct ClosableDestination<Destination>
where
	Destination: Observer,
{
	pub destination: Destination,
	pub closed: bool,
}

impl<Destination> From<Destination> for ClosableDestination<Destination>
where
	Destination: Observer,
{
	fn from(destination: Destination) -> Self {
		Self {
			destination,
			closed: false,
		}
	}
}

// Maybe this should be a shared subscriber?
pub struct SharedObserver<Destination>
where
	Destination: Observer,
{
	destination: Arc<Mutex<ClosableDestination<Destination>>>,
}

impl<Destination> SharedObserver<Destination>
where
	Destination: Observer,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: Arc::new(Mutex::new(ClosableDestination {
				destination,
				closed: false,
			})),
		}
	}

	pub fn new_from_shared(destination: Arc<Mutex<ClosableDestination<Destination>>>) -> Self {
		Self {
			destination: destination.clone(),
		}
	}

	/// Let's you check the shared observer for the duration of the callback
	pub fn read<F>(&mut self, mut viewer: F)
	where
		F: FnMut(&mut Destination),
	{
		viewer(&mut self.destination.lock().unwrap().destination)
	}
}

impl<Destination> Clone for SharedObserver<Destination>
where
	Destination: Observer,
{
	fn clone(&self) -> Self {
		Self {
			destination: self.destination.clone(),
		}
	}
}

impl<Destination> ObserverInput for SharedObserver<Destination>
where
	Destination: Observer,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> Observer for SharedObserver<Destination>
where
	Destination: Observer,
{
	fn next(&mut self, next: Self::In) {
		let mut lock = self.destination.lock().expect("lock is poisoned!");
		if !lock.closed {
			lock.destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		let mut lock = self.destination.lock().expect("lock is poisoned!");

		if !lock.closed {
			lock.closed = true;
			lock.destination.error(error);
		}
	}

	fn complete(&mut self) {
		let mut lock = self.destination.lock().expect("lock is poisoned!");
		if !lock.closed {
			lock.closed = true;
			lock.destination.complete();
		}
	}
}

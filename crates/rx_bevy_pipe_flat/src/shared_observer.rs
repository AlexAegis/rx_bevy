use std::sync::{Arc, RwLock};

use rx_bevy_observable::Observer;

pub struct ClosableDestination<Destination>
where
	Destination: Observer,
{
	destination: Destination,
	closed: bool,
}

pub struct SharedObserver<Destination>
where
	Destination: Observer,
{
	destination: Arc<RwLock<ClosableDestination<Destination>>>,
}

impl<Destination> SharedObserver<Destination>
where
	Destination: Observer,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: Arc::new(RwLock::new(ClosableDestination {
				destination,
				closed: false,
			})),
		}
	}

	pub fn new_from_shared(destination: &Arc<RwLock<ClosableDestination<Destination>>>) -> Self {
		Self {
			destination: destination.clone(),
		}
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

impl<Destination> Observer for SharedObserver<Destination>
where
	Destination: Observer,
{
	type In = Destination::In;
	type Error = Destination::Error;

	fn next(&mut self, next: Self::In) {
		// TODO: Maybe try with read access first? Or just use a mutex?
		let mut lock = self.destination.write().expect("lock is poisoned!");
		if !lock.closed {
			lock.destination.next(next);
		}
	}

	fn error(&mut self, error: Self::Error) {
		let mut lock = self.destination.write().expect("lock is poisoned!");

		if !lock.closed {
			lock.closed = true;
			lock.destination.error(error);
		}
	}

	fn complete(&mut self) {
		let mut lock = self.destination.write().expect("lock is poisoned!");
		if !lock.closed {
			println!("LOL2");
			lock.closed = true;
			lock.destination.complete();
		}
	}
}

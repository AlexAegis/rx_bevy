use crate::Observer;

///
pub trait Subscription {
	// type Destination: Observer;

	fn unsubscribe(&mut self);

	fn is_closed(&self) -> bool;

	// fn with_destination(destination: Self::Destination) -> Self;
}

impl Subscription for () {
	fn is_closed(&self) -> bool {
		false
	}

	fn unsubscribe(&mut self) {}
}

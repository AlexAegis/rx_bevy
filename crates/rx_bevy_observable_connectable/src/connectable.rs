use rx_bevy_observable::{Observable, Subscription};

pub trait Connectable: Observable {
	fn connect(&mut self) -> Subscription;
}

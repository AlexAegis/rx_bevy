use rx_bevy_core::{Observable, Subscription};

pub trait Connectable: Observable {
	fn connect(&mut self) -> Subscription;
}

use rx_bevy_core::{DropSubscription, Observable};

pub trait Connectable: Observable {
	fn connect(&mut self) -> DropSubscription;
}

use rx_bevy_observable::{Observable, Observer};

use crate::HigherOrderObserver;

pub struct HigherOrderForwarder<Fw, Destination>
where
	Fw: HigherOrderSubscriber,
	Destination: Observer,
{
	destination: Destination,
	operator: Fw,
	pub is_closed: bool,
}

impl<Fw, Destination> HigherOrderForwarder<Fw, Destination>
where
	Fw: HigherOrderSubscriber,
	Destination: Observer,
{
	pub fn new(destination: Destination, operator: Fw) -> Self {
		Self {
			destination,
			operator,
			is_closed: false,
		}
	}
}

pub trait HigherOrderSubscriber {
	type In;

	fn subscribe_on_next(&mut self, next: Self::In);
}

pub trait HigherOrderOperator {
	type OutObservable: Observable + Clone; // TODO: Remover CLone later as it's DEFNITELY NOT NEEDED, ONLY HERE BECAUSE OF THE TEMPORARY SUBJECT BASED IMPL
	type Subscriber: HigherOrderSubscriber;

	fn higher_order_operator_subscribe<
		Destination: 'static
			+ Observer<
				In = Self::OutObservable,
				// Error = <Self::OutObservable as Observable>::Error,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> HigherOrderForwarder<Self::Subscriber, Destination>;
}

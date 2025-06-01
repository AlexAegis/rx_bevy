use std::marker::PhantomData;

use rx_bevy_observable::{ObserverConnector, DynObserverConnector, Observer};
use rx_bevy_operator::{ForwardObserver, Operator};

#[derive(Debug)]
pub struct TapOperator<In, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	callback: Callback,
	_phantom_data: PhantomData<In>,
}

impl<In, Callback> Operator for TapOperator<In, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type In = In;
	type Out = In;

	type InternalSubscriber = Self;

	fn operator_subscribe<Destination: 'static + Observer<In = Self::Out>>(
		&mut self,
		destination: Destination,
	) -> rx_bevy_operator::ForwardObserver<Self::InternalSubscriber, Destination> {
		ForwardObserver::new(self.clone(), destination)
	}
}

impl<In, Callback> ObserverConnector for TapOperator<In, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	type In = In;
	type Out = In;

	fn push_forward<Destination: Observer<In = In>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		destination.on_push(next);
	}
}

impl<In, Callback> TapOperator<In, Callback>
where
	Callback: for<'a> Fn(&'a In),
{
	pub fn new(callback: Callback) -> Self {
		Self {
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Callback> Clone for TapOperator<In, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	fn clone(&self) -> Self {
		Self {
			callback: self.callback.clone(),
			_phantom_data: PhantomData,
		}
	}
}

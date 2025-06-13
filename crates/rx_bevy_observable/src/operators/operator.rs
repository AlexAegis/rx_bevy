use crate::{Forwarder, ObservableOutput, Observer, ObserverInput, Subscriber};

/// Every Operator is an Observer that can subscribe to an observable, and upon
/// subscription, returns it's own [OperatorObserver] that you can subscribe to.
/// Destination is the Observer that will get subscribed to this internal Observable.
pub trait Operator {
	type Fw: Forwarder;

	fn create_instance(&self) -> Self::Fw;

	#[inline]
	fn operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self::Fw as ObservableOutput>::Out,
				InError = <Self::Fw as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Fw, Destination> {
		Subscriber::new(destination, self.create_instance())
	}
}

impl<T> ObserverInput for T
where
	T: Operator,
{
	type In = <T::Fw as ObserverInput>::In;
	type InError = <T::Fw as ObserverInput>::InError;
}

impl<T> ObservableOutput for T
where
	T: Operator,
{
	type Out = <T::Fw as ObservableOutput>::Out;
	type OutError = <T::Fw as ObservableOutput>::OutError;
}

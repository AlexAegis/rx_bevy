use crate::{ObservableOutput, Observer, ObserverInput, Subscriber, SubscriberForwarder};

/// Every Operator is an Observer that can subscribe to an observable, and upon
/// subscription, returns it's own [OperatorObserver] that you can subscribe to.
/// Destination is the Observer that will get subscribed to this internal Observable.
pub trait Operator: ObserverInput + ObservableOutput {
	type Sub<D>: SubscriberForwarder<Destination = D>
		+ ObserverInput<In = Self::In, InError = Self::InError>
		+ ObservableOutput<Out = Self::Out, OutError = Self::OutError>;

	fn create_instance<Destination>(&self) -> Self::Sub<Destination>;

	#[inline]
	fn operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Sub<Destination>, Destination> {
		Subscriber::new(destination, self.create_instance::<Destination>())
	}
}

/*
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
*/

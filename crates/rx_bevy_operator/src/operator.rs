use rx_bevy_observable::{
	Forwarder, LiftedSubscriber, LiftingForwarder, Observable, Observer, Subscriber,
};

// OperatorIO OperatorInstanceFactory

/// Every Operator is an Observer that can subscribe to an observable, and upon
/// subscription, returns it's own [OperatorObserver] that you can subscribe to.
/// Destination is the Observer that will get subscribed to this internal Observable.
pub trait Operator {
	type Fw: Forwarder;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<In = <Self::Fw as Forwarder>::Out, Error = <Self::Fw as Forwarder>::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Fw, Destination>;
}

pub trait LiftingOperator {
	type Fw: LiftingForwarder;

	fn lifted_operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self::Fw as LiftingForwarder>::OutObservable,
				Error = <<Self::Fw as LiftingForwarder>::OutObservable as Observable>::Error,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> LiftedSubscriber<Self::Fw, Destination>;
}

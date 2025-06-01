use rx_bevy_observable::{ConnectorObserver, Observer};

use crate::ForwardObserver;

// OperatorIO OperatorInstanceFactory

/// Every Operator is an Observer that can subscribe to an observable, and upon
/// subscription, returns it's own [OperatorObserver] that you can subscribe to.
/// Destination is the Observer that will get subscribed to this internal Observable.
pub trait Operator {
	/// Input type of the operator
	type In;
	/// Output type of the operator
	type Out;

	type InternalSubscriber: ConnectorObserver<In = Self::In, Out = Self::Out>;

	fn operator_subscribe<Destination: 'static + Observer<Self::Out>>(
		&mut self,
		destination: Destination,
	) -> ForwardObserver<Self::InternalSubscriber, Destination>;
}

/// Many operators let the user define a function to be passed, this type ensures
/// they are clone-able which is required for instancing the operator.
pub trait OperatorCallback<In, Out>: Clone + Fn(In) -> Out {}
pub trait OperatorCallbackRef<In, Out>: Clone + for<'a> Fn(&'a In) -> Out {}

impl<T, In, Out> OperatorCallback<In, Out> for T where T: Clone + Fn(In) -> Out {}

pub trait OperatorCallbackOnce<In, Out>: Clone + FnOnce(In) -> Out {}

impl<T, In, Out> OperatorCallbackOnce<In, Out> for T where T: Clone + FnOnce(In) -> Out {}

pub trait OperatorCallbackMut<In, Out>: Clone + FnMut(In) -> Out {}
pub trait OperatorCallbackRefMuf<In, Out>: Clone + for<'a> FnMut(&'a In) -> Out {}

impl<T, In, Out> OperatorCallbackMut<In, Out> for T where T: Clone + FnMut(In) -> Out {}

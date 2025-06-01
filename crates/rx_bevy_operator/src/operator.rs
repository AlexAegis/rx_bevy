use rx_bevy_observable::{Observable, Observer, Subscription};

use super::{OperatorInstance, OperatorInstanceForwardObserver};

// OperatorIO OperatorInstanceFactory

/// Every Operator is an Observer that can subscribe to an observable, and upon
/// subscription, returns it's own [OperatorObserver] that you can subscribe to.
/// Destination is the Observer that will get subscribed to this internal Observable.
pub trait Operator {
	/// Input type of the operator
	type In;
	/// Output type of the operator
	type Out;

	/// The operators internal observer, that observes the source/upstream observable
	/// Its input is the operators output
	type Instance: OperatorInstance<In = Self::In, Out = Self::Out>;

	fn create_operator_instance(&self) -> Self::Instance;

	// TODO: Maybe this is a bad idea, sometimes it's not useful
	fn operate(&mut self, next: Self::In) -> Self::Out;
}

pub trait OperatorSubscribe: Operator {
	fn operator_subscribe<
		Source: Observable<Out = Self::In>,
		Destination: Observer<In = Self::Out>,
	>(
		self,
		source: Source,
		observer: Destination,
	) -> Subscription<
		OperatorInstanceForwardObserver<Self::In, Self::Out, Self::Instance, Destination>,
	>;
}

impl<T> OperatorSubscribe for T
where
	T: Operator,
{
	fn operator_subscribe<
		Source: Observable<Out = Self::In>,
		Destination: Observer<In = Self::Out>,
	>(
		self,
		mut source: Source,
		destination: Destination,
	) -> Subscription<
		OperatorInstanceForwardObserver<Self::In, Self::Out, Self::Instance, Destination>,
	> {
		let operator_internal_forwarder = self.create_operator_instance();
		let forward_observer =
			OperatorInstanceForwardObserver::new(operator_internal_forwarder, destination);
		source.subscribe(forward_observer)
	}
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

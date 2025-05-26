use crate::{observables::Observable, observers::Observer};

use super::{OperatorInstance, OperatorInstanceForwardObserver};

pub trait Operator<Source>:
	OperatorIO + OperatorInstanceFactory + OperatorWithSource + OperatorSource<Source>
{
}

pub trait OperatorIO {
	/// Input type of the operator
	type In;
	/// Output type of the operator
	type Out;
}

/// Every Operator is an Observer that can subscribe to an observable, and upon
/// subscription, returns it's own [OperatorObserver] that you can subscribe to.
/// Destination is the Observer that will get subscribed to this internal Observable.
pub trait OperatorInstanceFactory: OperatorIO {
	/// The operators internal observer, that observes the source/upstream observable
	/// Its input is the operators output
	type Instance: OperatorInstance<In = Self::In, Out = Self::Out>;
	fn create_operator_instance(&self) -> Self::Instance;
}

/// OperatorWithSource and OperatorSource<Source> are separate, otherwise the
/// pipe impl wouldn't work
pub trait OperatorWithSource: OperatorIO {
	/// The source observable this operators internal observer observes.
	/// Its output is the operators input
	type SourceObservable: Observable<Out = Self::In>;
}

pub trait OperatorSource<Source> {
	fn take_source_observable(&mut self) -> Option<Source>;
	fn replace_source(&mut self, source: Source) -> Option<Source>;
}

pub trait OperatorSubscribe: OperatorIO {
	fn operator_subscribe<Destination: Observer<In = Self::Out>>(self, observer: Destination);
}

impl<T> OperatorSubscribe for T
where
	T: OperatorInstanceFactory + OperatorWithSource + OperatorSource<T::SourceObservable>,
{
	fn operator_subscribe<Destination: Observer<In = Self::Out>>(
		mut self,
		destination: Destination,
	) {
		if let Some(source) = self.take_source_observable() {
			let operator_internal_forwarder = self.create_operator_instance();
			let forward_observer =
				OperatorInstanceForwardObserver::new(operator_internal_forwarder, destination);
			source.subscribe(forward_observer);
		}
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

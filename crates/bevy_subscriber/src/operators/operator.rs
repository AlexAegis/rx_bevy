use crate::{observables::Observable, observers::Observer};

/// Every Operator is an Observer that can subscribe to an observable, and upon
/// subscription, returns it's own [OperatorObserver] that you can subscribe to.
/// Destination is the Observer that will get subscribed to this internal Observable.
pub trait OperatorData<Destination>
where
	Destination: Observer<In = Self::Out>,
	Self: Sized,
{
	/// Input type of the operator
	type In;
	/// Output type of the operator
	type Out;

	/// The source observable this operators internal observer observes.
	/// Its output is the operators input
	type SourceObservable: Observable<Self::InternalOperatorObserver, Out = Self::In>;
	/// The operators internal observer, that observes the source/upstream observable
	/// Its input is the operators output
	type InternalOperatorObserver: Observer<In = Self::In>;

	fn into_observer(self, destination: Destination) -> Self::InternalOperatorObserver;
	fn take_source_observable(&mut self) -> Option<Self::SourceObservable>;
	fn replace_source(&mut self, source: Self::SourceObservable) -> Option<Self::SourceObservable>;
}

pub trait OperatorSubscribe<Destination> {
	fn operator_subscribe(self, observer: Destination);
}

impl<T, Destination, Out> OperatorSubscribe<Destination> for T
where
	T: OperatorData<Destination, Out = Out>,
	Destination: Observer<In = Out>,
{
	fn operator_subscribe(mut self, destination: Destination) {
		if let Some(source) = self.take_source_observable() {
			let operator_internal_observer = Self::into_observer(self, destination);
			source.internal_subscribe(operator_internal_observer);
		}
	}
}

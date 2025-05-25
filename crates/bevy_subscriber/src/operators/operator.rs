use crate::{observables::Observable, observers::Observer};

pub trait OperatorIO {
	/// Input type of the operator
	type In;
	/// Output type of the operator
	type Out;
}

/// Every Operator is an Observer that can subscribe to an observable, and upon
/// subscription, returns it's own [OperatorObserver] that you can subscribe to.
/// Destination is the Observer that will get subscribed to this internal Observable.
pub trait OperatorIntoObserver<Destination>
where
	Destination: Observer<In = Self::Out>,
	Self: Sized + OperatorIO,
{
	// TODO: Simplify? Maybe these could always just live inside the source/internal fields

	/// The source observable this operators internal observer observes.
	/// Its output is the operators input
	type SourceObservable: Observable<Self::InternalOperatorObserver, Out = Self::In>;
	/// The operators internal observer, that observes the source/upstream observable
	/// Its input is the operators output
	type InternalOperatorObserver: Observer<In = Self::In>;

	fn into_observer(self, destination: Destination) -> Self::InternalOperatorObserver;
}

pub trait OperatorSource<Source> {
	fn take_source_observable(&mut self) -> Option<Source>;
	fn replace_source(&mut self, source: Source) -> Option<Source>;
}

pub trait OperatorSubscribe<Destination> {
	fn subscribe(self, observer: Destination);
}

impl<T, Destination, Out> OperatorSubscribe<Destination> for T
where
	T: OperatorIntoObserver<Destination, Out = Out> + OperatorSource<T::SourceObservable>,
	Destination: Observer<In = Out>,
{
	fn subscribe(mut self, destination: Destination) {
		if let Some(source) = self.take_source_observable() {
			let operator_internal_observer = Self::into_observer(self, destination);
			source.subscribe(operator_internal_observer);
		}
	}
}

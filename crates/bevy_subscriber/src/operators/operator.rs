use crate::{
	observables::Observable,
	observers::{Forwarder, Observer},
};

pub trait OperatorIO {
	/// Input type of the operator
	type In;
	/// Output type of the operator
	type Out;
}

/// Every Operator is an Observer that can subscribe to an observable, and upon
/// subscription, returns it's own [OperatorObserver] that you can subscribe to.
/// Destination is the Observer that will get subscribed to this internal Observable.
pub trait OperatorWithForwarder: OperatorIO {
	/// The operators internal observer, that observes the source/upstream observable
	/// Its input is the operators output
	type Fwd: Forwarder<In = Self::In, Out = Self::Out>;
	fn into_forwarder(self) -> Self::Fwd;
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
	T: OperatorWithForwarder + OperatorWithSource + OperatorSource<T::SourceObservable>,
{
	fn operator_subscribe<Destination: Observer<In = Self::Out>>(
		mut self,
		destination: Destination,
	) {
		if let Some(source) = self.take_source_observable() {
			let operator_internal_forwarder = self.into_forwarder();
			let forward_observer = ForwardObserver::new(operator_internal_forwarder, destination);
			source.subscribe(forward_observer);
		}
	}
}

struct ForwardObserver<In, Out, F: Forwarder<In = In>, Destination: Observer<In = Out>> {
	forwarder: F,
	destination: Destination,
}

impl<In, Out, F, Destination> ForwardObserver<In, Out, F, Destination>
where
	F: Forwarder<In = In>,
	Destination: Observer<In = Out>,
{
	fn new(forwarder: F, destination: Destination) -> Self {
		Self {
			forwarder,
			destination,
		}
	}
}

impl<In, Out, F, Destination> Observer for ForwardObserver<In, Out, F, Destination>
where
	F: Forwarder<In = In, Out = Out>,
	Destination: Observer<In = Out>,
{
	type In = In;

	fn on_push(&mut self, value: Self::In) {
		self.forwarder.push_forward(value, &mut self.destination);
	}
}

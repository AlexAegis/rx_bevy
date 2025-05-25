use std::marker::PhantomData;

use crate::{observables::Observable, observers::Observer};

/// Also an automatic Observable
pub struct MapOperator<Source, In, Out, F>
where
	F: Fn(In) -> Out,
{
	pub source_observable: Option<Source>,
	pub transform: F,
	pub phantom_in: PhantomData<In>,
	pub phantom_out: PhantomData<Out>,
	// pub phantom_internal_observer: PhantomData<InternalObserver>,
}

impl<Source, Destination, In, Out, F> OperatorData<Destination> for MapOperator<Source, In, Out, F>
where
	F: Fn(In) -> Out,
	Source: Observable<MapObserver<Destination, F, In>, Out = In>,
	Destination: Observer<In = Out>,
{
	type In = In;
	type Out = Out;
	type SourceObservable = Source;
	type InternalOperatorObserver = MapObserver<Destination, F, In>;

	fn into_observer(self, destination: Destination) -> Self::InternalOperatorObserver {
		MapObserver {
			destination,
			transform: self.transform,
			_phantom_data_in: PhantomData,
		}
	}

	fn take_source_observable(&mut self) -> Option<Self::SourceObservable> {
		std::mem::take(&mut self.source_observable)
	}
}

/// Every Operator is an Observer that can subscribe to an observable, and upon
/// subscription, returns it's own [OperatorObserver] that you can subscribe to.
/// Destination is the Observer that will get subscribed to this internal Observable.
pub trait OperatorData<Destination>
where
	Destination: Observer<In = Self::Out>,
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
}
/*
pub trait Operator<Destination>:
	OperatorData<Destination, In = Self::In, Out = Self::Out> + OperatorSubscribe<Destination>
where
	Destination: Observer<In = Self::Out>,
{
	type In;
	type Out;
}
*/
/*
impl<Source, Subscriber, In, Out, F> Observable<Subscriber, Out> for MapOperator<Source, In, Out, F>
where
	F: Fn(In) -> Out,
	Source: Observable<MapObserver<Subscriber, F>, In>,
	Subscriber: Observer<Out>,
{
	fn internal_subscribe(mut self, observer: Subscriber) {
		if let Some(source) = std::mem::take(&mut self.source) {
			let observer = Self::into_observer(self, observer);
			source.internal_subscribe(observer);
		}
	}
}*/

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

pub trait IsOperator {}

impl<Source, In, Out, F> IsOperator for MapOperator<Source, In, Out, F> where F: Fn(In) -> Out {}
impl<Source, In, Out, F> IsOperator for &MapOperator<Source, In, Out, F> where F: Fn(In) -> Out {}
impl<Source, In, Out, F> IsOperator for &mut MapOperator<Source, In, Out, F> where F: Fn(In) -> Out {}

impl<Source, Destination, In, Out, F> Observable<Destination> for MapOperator<Source, In, Out, F>
where
	F: Fn(In) -> Out,
	Destination: Observer<In = Out>,
	Source: Observable<MapObserver<Destination, F, In>, Out = In>,
{
	type Out = Out;

	fn internal_subscribe(self, observer: Destination) {
		self.operator_subscribe(observer);
	}
}

/*
impl<T, Destination, In, Out> Observable<Destination> for T
where
	T: OperatorData<Destination, In = In, Out = Out> + OperatorSubscribe<Destination> + IsOperator, // [`In`] does not need to be constrained
	Destination: Observer<In = Out>,
{
	type Out = Out;

	fn internal_subscribe(self, observer: Destination) {
		self.operator_subscribe(observer);
	}
}*/

pub struct MapObserver<Destination, F, In> {
	destination: Destination,
	transform: F,
	_phantom_data_in: PhantomData<In>,
}

impl<In, Out, F, Destination> Observer for MapObserver<Destination, F, In>
where
	F: Fn(In) -> Out,
	Destination: Observer<In = Out>,
{
	type In = In;

	fn on_push(&mut self, value: Self::In) {
		let result = (self.transform)(value);
		self.destination.on_push(result);
	}
}

use std::marker::PhantomData;

use crate::{observables::Observable, observers::Observer};

use super::{OperatorData, OperatorSubscribe};

pub struct MapOperator<Source, In, Out, F> {
	pub source_observable: Option<Source>,
	pub transform: F,
	pub phantom_in: PhantomData<In>,
	pub phantom_out: PhantomData<Out>,
}

impl<Source, In, Out, F> MapOperator<Source, In, Out, F> {
	pub fn new(source: Source, transform: F) -> Self {
		Self {
			phantom_in: PhantomData,
			phantom_out: PhantomData,
			source_observable: Some(source),
			transform,
		}
	}
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

	fn replace_source(&mut self, source: Self::SourceObservable) -> Option<Self::SourceObservable> {
		self.source_observable.replace(source)
	}
}

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

/// TODO: Make generic or macro
impl<Destination, Source, In, Out, F> Observable<Destination> for MapOperator<Source, In, Out, F>
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

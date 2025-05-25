use std::marker::PhantomData;

use crate::{
	observables::{Observable, ObservableWithOperators},
	observers::{Forwarder, Observer},
};

use super::{
	OperatorIO, OperatorSource, OperatorSubscribe, OperatorWithForwarder, OperatorWithSource,
};

pub struct MapOperator<Source, In, Out, F> {
	pub source_observable: Option<Source>,
	pub transform: F,
	pub phantom_in: PhantomData<In>,
	pub phantom_out: PhantomData<Out>,
}

impl<Source, In, Out, F> OperatorWithForwarder for MapOperator<Source, In, Out, F>
where
	F: Fn(In) -> Out,
{
	type Fwd = MapForwarder<F, In, Out>;

	fn into_forwarder(self) -> Self::Fwd {
		MapForwarder {
			transform: self.transform,
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
		}
	}
}

impl<Source, In, Out, F> OperatorWithSource for MapOperator<Source, In, Out, F>
where
	Source: Observable<Out = Self::In>,
	F: Fn(In) -> Out,
{
	type SourceObservable = Source;
}

impl<Source, In, Out, F> MapOperator<Source, In, Out, F> {
	pub fn new(transform: F) -> Self {
		Self {
			phantom_in: PhantomData,
			phantom_out: PhantomData,
			source_observable: None,
			transform,
		}
	}

	pub fn new_with_source(source: Source, transform: F) -> Self {
		Self {
			phantom_in: PhantomData,
			phantom_out: PhantomData,
			source_observable: Some(source),
			transform,
		}
	}
}

impl<Source, In, Out, F> OperatorIO for MapOperator<Source, In, Out, F> {
	type In = In;
	type Out = Out;
}

/// TODO: Could be part of the macro with a #[source_observable] field attribute for Optional<Source>'s to specify where it is
impl<Source, In, Out, F> OperatorSource<Source> for MapOperator<Source, In, Out, F> {
	fn take_source_observable(&mut self) -> Option<Source> {
		std::mem::take(&mut self.source_observable)
	}

	fn replace_source(&mut self, source: Source) -> Option<Source> {
		self.source_observable.replace(source)
	}
}

pub struct MapForwarder<F, In, Out> {
	// destination: Destination,
	transform: F,
	_phantom_data_in: PhantomData<In>,
	_phantom_data_out: PhantomData<Out>,
}

impl<F, In, Out> Forwarder for MapForwarder<F, In, Out>
where
	F: Fn(In) -> Out,
{
	type In = In;
	type Out = Out;

	fn push_forward<Destination: Observer<In = Out>>(
		&mut self,
		value: Self::In,
		destination: &mut Destination,
	) {
		let result = (self.transform)(value);
		destination.on_push(result);
	}
}

/// TODO: Make generic or macro
impl<Source, In, Out, F> Observable for MapOperator<Source, In, Out, F>
where
	F: Fn(In) -> Out,
	Source: Observable<Out = In>,
{
	type Out = Out;

	fn subscribe<Destination: Observer<In = Out>>(self, observer: Destination) {
		OperatorSubscribe::operator_subscribe(self, observer);
	}
}

impl<Source, In, Out, F> ObservableWithOperators<Out> for MapOperator<Source, In, Out, F>
where
	F: Fn(In) -> Out,
	Source: Observable<Out = In>,
{
}

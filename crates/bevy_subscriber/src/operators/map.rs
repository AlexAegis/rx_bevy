use std::marker::PhantomData;

use crate::{
	observables::{Observable, ObservableExtensionPipe},
	observers::Observer,
};

use super::{
	OperatorCallback, OperatorIO, OperatorInstance, OperatorInstanceFactory, OperatorSource,
	OperatorSubscribe, OperatorWithSource,
};

pub struct MapOperator<Source, In, Out, F> {
	pub source_observable: Option<Source>,
	pub callback: F,
	pub phantom_in: PhantomData<In>,
	pub phantom_out: PhantomData<Out>,
}

impl<Source, In, Out, F> OperatorInstanceFactory for MapOperator<Source, In, Out, F>
where
	F: OperatorCallback<In, Out>,
{
	type Instance = MapOperatorInstance<F, Self::In, Self::Out>;

	fn create_operator_instance(&self) -> Self::Instance {
		MapOperatorInstance {
			callback: self.callback.clone(),
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
			callback: transform,
		}
	}

	pub fn new_with_source(source: Source, transform: F) -> Self {
		Self {
			phantom_in: PhantomData,
			phantom_out: PhantomData,
			source_observable: Some(source),
			callback: transform,
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

pub struct MapOperatorInstance<F, In, Out> {
	callback: F,
	_phantom_data_in: PhantomData<In>,
	_phantom_data_out: PhantomData<Out>,
}

impl<F, In, Out> OperatorInstance for MapOperatorInstance<F, In, Out>
where
	F: OperatorCallback<In, Out>,
{
	type In = In;
	type Out = Out;

	fn push_forward<Destination: Observer<In = Out>>(
		&mut self,
		value: Self::In,
		destination: &mut Destination,
	) {
		let result = (self.callback)(value);
		destination.on_push(result);
	}
}

/// TODO: Make generic or macro
impl<Source, In, Out, F> Observable for MapOperator<Source, In, Out, F>
where
	F: OperatorCallback<In, Out>,
	Source: Observable<Out = In>,
{
	type Out = Out;

	fn subscribe<Destination: Observer<In = Out>>(self, observer: Destination) {
		OperatorSubscribe::operator_subscribe(self, observer);
	}
}

pub trait ObservableExtensionMap<Out>: Observable<Out = Out> + Sized {
	fn map<NextOut, F: Fn(Out) -> NextOut>(
		self,
		transform: F,
	) -> MapOperator<Self, Out, NextOut, F> {
		MapOperator::new_with_source(self, transform)
	}
}

impl<T, Out> ObservableExtensionMap<Out> for T where T: Observable<Out = Out> {}

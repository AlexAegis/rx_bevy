use std::marker::PhantomData;

use crate::{observables::Observable, observers::Observer};

use super::{
	Operator, OperatorCallbackRef, OperatorIO, OperatorInstance, OperatorInstanceFactory,
	OperatorSource, OperatorSubscribe, OperatorWithSource,
};

pub struct TapOperator<Source, In, Callback>
where
	Callback: OperatorCallbackRef<In, ()>,
{
	source_observable: Option<Source>,
	callback: Callback,
	_phantom_data: PhantomData<In>,
}

impl<Source, In, Callback> TapOperator<Source, In, Callback>
where
	Callback: OperatorCallbackRef<In, ()>,
{
	pub fn new(callback: Callback) -> Self {
		Self {
			_phantom_data: PhantomData,
			callback,
			source_observable: None,
		}
	}

	pub fn new_with_source(source: Source, callback: Callback) -> Self {
		Self {
			_phantom_data: PhantomData,
			callback,
			source_observable: Some(source),
		}
	}
}

impl<Source, In, Callback> Operator<Source> for TapOperator<Source, In, Callback>
where
	Callback: OperatorCallbackRef<In, ()>,
	Source: Observable<Out = Self::In>,
{
}

impl<Source, In, Callback> OperatorIO for TapOperator<Source, In, Callback>
where
	Callback: OperatorCallbackRef<In, ()>,
{
	type In = In;
	type Out = In;
}

impl<Source, In, Callback> OperatorWithSource for TapOperator<Source, In, Callback>
where
	Source: Observable<Out = Self::In>,
	Callback: OperatorCallbackRef<In, ()>,
{
	type SourceObservable = Source;
}

impl<Source, In, Callback> OperatorSource<Source> for TapOperator<Source, In, Callback>
where
	Callback: OperatorCallbackRef<In, ()>,
{
	fn replace_source(&mut self, source: Source) -> Option<Source> {
		self.source_observable.replace(source)
	}

	fn take_source_observable(&mut self) -> Option<Source> {
		std::mem::take(&mut self.source_observable)
	}
}

pub struct TapOperatorInstance<In, Callback>
where
	Callback: OperatorCallbackRef<In, ()>,
{
	callback: Callback,
	_phantom_data: PhantomData<In>,
}

impl<In, Callback> OperatorInstance for TapOperatorInstance<In, Callback>
where
	Callback: OperatorCallbackRef<In, ()>,
{
	type In = In;
	type Out = In;

	fn push_forward<Destination: Observer<In = Self::Out>>(
		&mut self,
		value: Self::In,
		destination: &mut Destination,
	) {
		(self.callback)(&value);
		destination.on_push(value);
	}
}

impl<Source, In, Callback> OperatorInstanceFactory for TapOperator<Source, In, Callback>
where
	Callback: OperatorCallbackRef<In, ()>,
{
	type Instance = TapOperatorInstance<In, Callback>;

	fn create_operator_instance(&self) -> Self::Instance {
		Self::Instance {
			_phantom_data: PhantomData,
			callback: self.callback.clone(),
		}
	}
}

impl<Source, In, F> Observable for TapOperator<Source, In, F>
where
	F: OperatorCallbackRef<In, ()>,
	Source: Observable<Out = In>,
{
	type Out = In;

	fn subscribe<Destination: Observer<In = In>>(self, observer: Destination) {
		OperatorSubscribe::operator_subscribe(self, observer);
	}
}

pub trait ObservableExtensionTap<Out>: Observable<Out = Out> + Sized {
	fn tap<Callback: OperatorCallbackRef<Out, ()>>(
		self,
		callback: Callback,
	) -> TapOperator<Self, Out, Callback> {
		TapOperator::new_with_source(self, callback)
	}
}

impl<T, Out> ObservableExtensionTap<Out> for T where T: Observable<Out = Out> {}

use std::marker::PhantomData;

use rx_bevy_observable::{Observable, ObservableOutput, Observer, Subscription};

pub fn deferred_observable<F, Source>(observable_creator: F) -> DeferredObservable<F, Source>
where
	Source: Observable,
	F: Fn() -> Source,
{
	DeferredObservable::new(observable_creator)
}

/// Defers the creation of its source [Observable] until subscribe
/// TODO: move to core or its own crate
pub struct DeferredObservable<F, Source>
where
	Source: Observable,
	F: Fn() -> Source,
{
	observable_creator: F,
	_phantom_data: PhantomData<Source>,
}

impl<F, Source> DeferredObservable<F, Source>
where
	Source: Observable,
	F: Fn() -> Source,
{
	pub fn new(observable_creator: F) -> Self {
		Self {
			observable_creator,
			_phantom_data: PhantomData,
		}
	}
}

impl<F, Source> Observable for DeferredObservable<F, Source>
where
	Source: Observable,
	F: Fn() -> Source,
{
	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		let mut source = (self.observable_creator)();
		source.subscribe(destination)
	}
}

impl<F, Source> ObservableOutput for DeferredObservable<F, Source>
where
	Source: Observable,
	F: Fn() -> Source,
{
	type Out = Source::Out;
	type OutError = Source::OutError;
}

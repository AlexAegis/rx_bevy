use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, DropSubscription, Observable, ObservableOutput, Observer, SignalContext,
	UpgradeableObserver,
};

pub fn deferred_observable<F, Source>(observable_creator: F) -> DeferredObservable<F, Source>
where
	Source: Observable,
	F: Clone + Fn() -> Source,
{
	DeferredObservable::new(observable_creator)
}

/// Defers the creation of its source [Observable] until subscribe
#[derive(Clone)]
pub struct DeferredObservable<F, Source>
where
	Source: Observable,
	F: Clone + Fn() -> Source,
{
	observable_creator: F,
	_phantom_data: PhantomData<Source>,
}

impl<F, Source> DeferredObservable<F, Source>
where
	Source: Observable,
	F: Clone + Fn() -> Source,
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
	<Source::Subscription as SignalContext>::Context: DropContext,
	F: Clone + Fn() -> Source,
{
	type Subscription = Source::Subscription;

	fn subscribe<
		Destination: 'static
			+ UpgradeableObserver<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Source::Subscription as SignalContext>::Context,
			>,
	>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> Self::Subscription {
		let subscriber = destination.upgrade();
		let mut source = (self.observable_creator)();
		source.subscribe(subscriber, context)
	}
}

impl<F, Source> ObservableOutput for DeferredObservable<F, Source>
where
	Source: Observable,
	F: Clone + Fn() -> Source,
{
	type Out = Source::Out;
	type OutError = Source::OutError;
}

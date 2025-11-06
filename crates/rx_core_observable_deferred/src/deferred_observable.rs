use core::marker::PhantomData;

use rx_core_traits::{
	Observable, ObservableOutput, PrimaryCategoryObservable, SubscriptionContext,
	UpgradeableObserver, WithPrimaryCategory, WithSubscriptionContext,
};

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

impl<F, Source> WithSubscriptionContext for DeferredObservable<F, Source>
where
	Source: Observable,
	F: Clone + Fn() -> Source,
{
	type Context = Source::Context;
}

impl<F, Source> WithPrimaryCategory for DeferredObservable<F, Source>
where
	Source: Observable,
	F: Clone + Fn() -> Source,
{
	type PrimaryCategory = PrimaryCategoryObservable;
}

impl<F, Source> Observable for DeferredObservable<F, Source>
where
	Source: Observable,
	F: Clone + Fn() -> Source,
{
	type Subscription = Source::Subscription;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let destination = observer.upgrade();
		let mut source = (self.observable_creator)();
		source.subscribe(destination, context)
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

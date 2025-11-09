use core::marker::PhantomData;

use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Observable, SubscriptionContext, UpgradeableObserver};

/// Defers the creation of its source [Observable] until subscribe
#[derive(RxObservable, Clone)]
#[rx_out(Source::Out)]
#[rx_out_error(Source::OutError)]
#[rx_context(Source::Context)]
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

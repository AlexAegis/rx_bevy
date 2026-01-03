use core::marker::PhantomData;

use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Observable, Subscriber, UpgradeableObserver};

/// Defers the creation of its source [Observable] until subscribe
#[derive(RxObservable)]
#[rx_out(Source::Out)]
#[rx_out_error(Source::OutError)]
pub struct DeferredObservable<F, Source>
where
	Source: Observable,
	F: FnMut() -> Source,
{
	observable_creator: F,
	_phantom_data: PhantomData<Source>,
}

impl<F, Source> DeferredObservable<F, Source>
where
	Source: Observable,
	F: FnMut() -> Source,
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
	F: FnMut() -> Source,
{
	type Subscription<Destination>
		= Source::Subscription<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let destination = observer.upgrade();
		let mut source = (self.observable_creator)();
		source.subscribe(destination)
	}
}

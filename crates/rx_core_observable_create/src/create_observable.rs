use core::marker::PhantomData;

use rx_core_common::{
	Never, Observable, PhantomInvariant, SharedSubscription, Signal, Subscriber,
	SubscriptionLikeExtensionIntoShared, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;

/// # CreateObservable
///
/// Uses a user-supplied producer to drive the destination subscriber.
/// The producer is cloned per subscription and called exactly once.
#[derive(RxObservable, Clone, Debug)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct CreateObservable<Producer, Out, OutError = Never>
where
	Out: Signal,
	OutError: Signal,
	Producer: Clone + FnOnce(&mut dyn Subscriber<In = Out, InError = OutError>),
{
	producer: Producer,
	_phantom: PhantomInvariant<(Out, OutError)>,
}

impl<Producer, Out, OutError> CreateObservable<Producer, Out, OutError>
where
	Out: Signal,
	OutError: Signal,
	Producer: Clone + FnOnce(&mut dyn Subscriber<In = Out, InError = OutError>),
{
	pub fn new(producer: Producer) -> Self {
		Self {
			producer,
			_phantom: PhantomData,
		}
	}
}

impl<Producer, Out, OutError> Observable for CreateObservable<Producer, Out, OutError>
where
	Out: Signal,
	OutError: Signal,
	Producer: Clone + FnOnce(&mut dyn Subscriber<In = Out, InError = OutError>),
{
	type Subscription<Destination>
		= SharedSubscription
	where
		Destination: 'static + rx_core_common::Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let mut destination = observer.upgrade();
		(self.producer.clone())(&mut destination);
		destination.into_shared()
	}
}

use std::marker::PhantomData;

use bevy_ecs::event::Event;
use derive_where::derive_where;
use rx_bevy_context::RxBevyContext;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Never, Observable, Subscriber, SubscriptionContext, UpgradeableObserver};

use crate::MessageSubscription;

#[derive_where(Default)]
#[derive(RxObservable)]
#[rx_out(M)]
#[rx_out_error(Never)]
#[rx_context(RxBevyContext)]
pub struct MessageObservable<M>
where
	M: Event + Clone, // TODO(bevy-0.17): use the message trait
{
	_phantom_data: PhantomData<M>,
}

impl<M> Observable for MessageObservable<M>
where
	M: Event + Clone,
{
	type Subscription<Destination>
		= MessageSubscription<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		MessageSubscription::new(destination.upgrade())
	}
}

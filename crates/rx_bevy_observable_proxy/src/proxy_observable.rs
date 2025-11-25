use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, schedule::ScheduleLabel};
use rx_bevy_common::Clock;
use rx_bevy_context::RxBevyContext;
use rx_core_macro_observable_derive::RxObservable;

use rx_core_traits::{
	Observable, SignalBound, Subscriber, SubscriptionContext, UpgradeableObserver,
};

use super::proxy_subscription::ProxySubscription;

/// An observable that sources its events by just subscribing to another
/// entity.
#[derive(RxObservable, Clone, Debug)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(RxBevyContext)]
pub struct ProxyObservable<In, InError, S, C>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	S: ScheduleLabel,
	C: Clock,
{
	target_observable_entity: Entity,
	_phantom_data: PhantomData<(In, InError, S, C)>,
}

impl<In, InError, S, C> ProxyObservable<In, InError, S, C>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	S: ScheduleLabel,
	C: Clock,
{
	pub fn new(target_observable_entity: Entity) -> Self {
		Self {
			target_observable_entity,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, S, C> Observable for ProxyObservable<In, InError, S, C>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	S: ScheduleLabel,
	C: Clock,
{
	type Subscription<Destination>
		= ProxySubscription<Destination, S, C>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		ProxySubscription::new(
			self.target_observable_entity,
			destination.upgrade(),
			context,
		)
	}
}

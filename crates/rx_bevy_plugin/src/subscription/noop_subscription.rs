use std::marker::PhantomData;

use derive_where::derive_where;

use rx_bevy_common_bounds::SignalBound;
use rx_bevy_core::ObservableOutput;

use crate::{CommandSubscriber, RxSubscription, SubscriptionChannelHandlerRegistrationContext};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// A No-op subscription, not scheduled, doesn't do anything but provide
/// type safety for observables that aren't scheduled. Use this if your
/// [ObservableComponent] does not need any scheduling, aka it can't produce
/// new events on its own, only when subscribed to.
#[derive_where(Default)]
#[derive(Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct NoopSubscription<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError> ObservableOutput for NoopSubscription<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	type Out = Out;
	type OutError = OutError;
}

impl<Out, OutError> RxSubscription for NoopSubscription<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	const SCHEDULED: bool = false;

	fn register_subscription_channel_handlers<'a, 'w, 's>(
		&mut self,
		_hooks: SubscriptionChannelHandlerRegistrationContext<'a, 'w, 's, Self>,
	) {
		// No hooks are registered
	}

	/// Still gets called, doesn't need to do anything
	fn unsubscribe(&mut self, _subscriber: CommandSubscriber<Self::Out, Self::OutError>) {}
}

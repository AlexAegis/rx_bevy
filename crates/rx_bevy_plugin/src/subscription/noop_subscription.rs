use std::marker::PhantomData;

use derive_where::derive_where;

use rx_bevy_observable::{ObservableOutput, Tick};

use crate::{CommandSubscriber, ScheduledSubscription, SignalBound};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// A No-op subscription, not scheduled, doesn't do anything but provide
/// type safety for observables that aren't scheduled. Use this if your
/// [ObservableComponent] does not need any scheduling, aka it can't produce
/// new events on its own, only when subscribed to.
#[derive_where(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
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

impl<Out, OutError> ScheduledSubscription for NoopSubscription<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	const SCHEDULED: bool = false;

	fn on_tick(&mut self, _tick: Tick, _context: CommandSubscriber<Self::Out, Self::OutError>) {
		unreachable!()
	}

	/// Still gets called, doesn't need to do anything
	fn unsubscribe(&mut self, _subscriber: CommandSubscriber<Self::Out, Self::OutError>) {}
}

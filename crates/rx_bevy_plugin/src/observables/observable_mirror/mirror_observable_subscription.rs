use bevy_ecs::entity::Entity;
use rx_bevy_observable::{ObservableOutput, Tick};
use std::marker::PhantomData;

use crate::{ObservableSignalBound, ScheduledSubscription};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ObservableMirrorSubscription<Out, OutError>
where
	Out: ObservableSignalBound,
	OutError: ObservableSignalBound,
{
	upstream_source: Entity,
	_phantom_pain: PhantomData<(Out, OutError)>,
}

impl<Out, OutError> ObservableMirrorSubscription<Out, OutError>
where
	Out: ObservableSignalBound,
	OutError: ObservableSignalBound,
{
	pub fn new(upstream_source: Entity) -> Self {
		Self {
			upstream_source,
			_phantom_pain: PhantomData,
		}
	}
}

impl<Out, OutError> ObservableOutput for ObservableMirrorSubscription<Out, OutError>
where
	Out: ObservableSignalBound,
	OutError: ObservableSignalBound,
{
	type Out = Out;
	type OutError = OutError;
}

impl<Out, OutError> ScheduledSubscription for ObservableMirrorSubscription<Out, OutError>
where
	Out: ObservableSignalBound,
	OutError: ObservableSignalBound,
{
	/// No need, just mirror whatever is coming in
	const SCHEDULED: bool = false;

	fn on_tick(
		&mut self,
		_tick: Tick,
		_subscriber: crate::CommandSubscriber<Self::Out, Self::OutError>,
	) {
		unreachable!()
	}

	fn unsubscribe(&mut self, _subscriber: crate::CommandSubscriber<Self::Out, Self::OutError>) {}
}

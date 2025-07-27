use bevy_ecs::{component::Component, entity::Entity};
use rx_bevy_observable::ObservableOutput;
use std::marker::PhantomData;

use crate::{
	CommandSubscriber, ObservableComponent, ObservableMirrorSubscription,
	ObservableOnInsertContext, SignalBound, Subscribe,
};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Component, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct MirrorObservableComponent<Out, OutError> {
	upstream_source: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_pain: PhantomData<(Out, OutError)>,
}

impl<Out, OutError> MirrorObservableComponent<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	pub fn new(upstream_source: Entity) -> Self {
		Self {
			upstream_source,
			_phantom_pain: PhantomData,
		}
	}
}

impl<Out, OutError> ObservableOutput for MirrorObservableComponent<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	type Out = Out;
	type OutError = OutError;
}

impl<Out, OutError> ObservableComponent for MirrorObservableComponent<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	/// There is no point to mirror something from the same entity, it's already there
	const CAN_SELF_SUBSCRIBE: bool = false;

	type Subscription = ObservableMirrorSubscription<Out, OutError>;

	fn on_insert(&mut self, _context: ObservableOnInsertContext) {}

	fn on_subscribe(
		&mut self,
		_subscriber: CommandSubscriber<Self::Out, Self::OutError>,
		_subscribe_event: &Subscribe<Self::Out, Self::OutError>,
	) -> Self::Subscription {
		ObservableMirrorSubscription::new(self.upstream_source)
	}
}

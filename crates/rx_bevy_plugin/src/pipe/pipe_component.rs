use bevy_ecs::component::{Component, Mutable, StorageType};
use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, ObserverInput, Operator};

use crate::{
	CommandSubscriber, ObservableOnInsertContext, OperatorComponent, PipeSubscriber,
	RelativeEntity, SignalBound, SubscriberContext, observable_on_remove_hook,
	operator_on_insert_hook,
};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	source: RelativeEntity,
	operator: Op,
}

impl<Op> PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	pub fn new(source: RelativeEntity, operator: Op) -> Self {
		Self { source, operator }
	}
}

impl<Op> Component for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	const STORAGE_TYPE: StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
		hooks.on_insert(operator_on_insert_hook::<Self>);
		hooks.on_remove(observable_on_remove_hook::<<Self as OperatorComponent>::Subscriber>);
	}
}

impl<Op> ObserverInput for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	type In = Op::In;
	type InError = Op::InError;
}

impl<Op> ObservableOutput for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

impl<Op> OperatorComponent for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	fn get_source(&self) -> RelativeEntity {
		self.source
	}

	type Subscriber = PipeSubscriber<Op>;

	fn on_insert(&mut self, _context: ObservableOnInsertContext) {}

	/// The subscription creates a new observer entity, that entity should have the subscriber on it.
	fn on_subscribe(
		&mut self,
		subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) -> Self::Subscriber {
		PipeSubscriber::<Op>::new(self.operator.operator_subscribe(subscriber.downgrade()))
	}
}

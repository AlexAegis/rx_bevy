use std::marker::PhantomData;

use bevy_ecs::{
	component::Component,
	system::{Commands, IntoSystem, SystemId},
};
use rx_bevy_observable::Tick;

use crate::{RxSignal, RxSubscriber, SignalBound};

use derive_where::derive_where;

#[derive(Component)]
#[derive_where(Default)]
#[cfg_attr(feature = "debug", derive_where(Debug))]
pub struct SubscriberHooks<Sub>
where
	Sub: RxSubscriber,
	Sub::In: SignalBound,
	Sub::InError: SignalBound,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub(crate) on_tick:
		Option<SystemId<bevy_ecs::system::In<Tick>, Vec<RxSignal<Sub::Out, Sub::OutError>>>>,
	pub(crate) on_signal: Option<
		SystemId<
			bevy_ecs::system::In<RxSignal<Sub::In, Sub::InError>>,
			Vec<RxSignal<Sub::Out, Sub::OutError>>,
		>,
	>,
	_phantom_data: PhantomData<Sub>,
}

impl<Sub> SubscriberHooks<Sub>
where
	Sub: RxSubscriber,
	Sub::In: SignalBound,
	Sub::InError: SignalBound,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn upgrade<'a, 'w, 's>(
		&'a mut self,
		commands: &'a mut Commands<'w, 's>,
	) -> SubscriberHookRegistrationContext<'a, 'w, 's, Sub> {
		SubscriberHookRegistrationContext::<'a, 'w, 's, Sub> {
			commands,
			on_tick: &mut self.on_tick,
			on_signal: &mut self.on_signal,
			_phantom_data: PhantomData,
		}
	}
}

#[cfg_attr(feature = "debug", derive_where(Debug))]
pub struct SubscriberHookRegistrationContext<'a, 'w, 's, Sub>
where
	Sub: RxSubscriber,
	Sub::In: SignalBound,
	Sub::InError: SignalBound,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	#[cfg_attr(feature = "debug", derive_where(skip))]
	commands: &'a mut Commands<'w, 's>,
	pub(crate) on_tick: &'a mut Option<
		SystemId<bevy_ecs::system::In<Tick>, Vec<RxSignal<Sub::Out, Sub::OutError>>>,
	>,
	pub(crate) on_signal: &'a mut Option<
		SystemId<
			bevy_ecs::system::In<RxSignal<Sub::In, Sub::InError>>,
			Vec<RxSignal<Sub::Out, Sub::OutError>>,
		>,
	>,
	_phantom_data: PhantomData<Sub>,
}

impl<'a, 'w, 's, Sub> SubscriberHookRegistrationContext<'a, 'w, 's, Sub>
where
	Sub: RxSubscriber,
	Sub::In: SignalBound,
	Sub::InError: SignalBound,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn register_on_tick<M>(
		&mut self,
		system: impl IntoSystem<bevy_ecs::system::In<Tick>, Vec<RxSignal<Sub::Out, Sub::OutError>>, M>
		+ 'static,
	) {
		let system_id = self.commands.register_system(system);
		if let Some(old_system) = self.on_tick.replace(system_id) {
			self.commands.entity(old_system.entity()).despawn();
		}
	}

	pub fn register_on_signal<M>(
		&mut self,
		system: impl IntoSystem<
			bevy_ecs::system::In<RxSignal<Sub::In, Sub::InError>>,
			Vec<RxSignal<Sub::Out, Sub::OutError>>,
			M,
		> + 'static,
	) {
		let system_id = self.commands.register_system(system);
		if let Some(old_system) = self.on_signal.replace(system_id) {
			self.commands.entity(old_system.entity()).despawn();
		}
	}

	pub fn downgrade(self) -> SubscriberHooks<Sub> {
		SubscriberHooks {
			on_signal: *self.on_signal,
			on_tick: *self.on_tick,
			_phantom_data: PhantomData,
		}
	}
}

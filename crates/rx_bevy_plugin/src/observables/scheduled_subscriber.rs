use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, system::Commands};
use rx_bevy::{ObservableOutput, ObserverInput};

use crate::{DebugBound, RxComplete, RxError, RxNext, RxTick, SubscriptionOnTickContext};

// TODO: Should be schedulable, probably from the Subscribe event, like schedule asap, once per frame, and time (maybe two, one ticked when AT LEAST a time passes, or when the current frame is expected to end after that limit)
pub trait ScheduledSubscription: ObservableOutput + DebugBound
where
	Self: Send + Sync,
	Self::Out: Send + Sync,
	Self::OutError: Send + Sync,
{
	/// When set to false, the subscription will not be ticked at all.
	const TICKABLE: bool = true;

	// /// Checked on every tick, and the [RxScheduler] will not call `tick` if
	// /// this returns false. Can be used as filter and/or to advance timers
	// fn can_tick_now(&mut self, _event: &RxTick) -> bool {
	// 	true
	// }

	fn on_event(&mut self, event: RxNext<Self::Out>, context: SubscriptionOnTickContext);
	fn on_tick(&mut self, event: &RxTick, context: SubscriptionOnTickContext);
}

impl ScheduledSubscription for () {
	fn on_event(&mut self, _event: RxNext<Self::Out>, _context: SubscriptionOnTickContext) {}

	fn on_tick(&mut self, _event: &RxTick, _context: SubscriptionOnTickContext) {}
}

pub struct CommandObserver<'a, 'w, 's, In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	commands: &'a mut Commands<'w, 's>,
	destination: Entity,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<'a, 'w, 's, In, InError> CommandObserver<'a, 'w, 's, In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	pub fn new(commands: &'a mut Commands<'w, 's>, destination: Entity) -> Self {
		Self {
			commands,
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<'a, 'w, 's, In, InError> ObserverInput for CommandObserver<'a, 'w, 's, In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	type In = In;
	type InError = InError;
}

impl<'a, 'w, 's, In, InError> rx_bevy::Observer for CommandObserver<'a, 'w, 's, In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	fn next(&mut self, next: Self::In) {
		self.commands
			.trigger_targets(RxNext(next), self.destination);
	}

	fn error(&mut self, error: Self::InError) {
		self.commands
			.trigger_targets(RxError(error), self.destination);
	}

	fn complete(&mut self) {
		self.commands.trigger_targets(RxComplete, self.destination);
	}
}

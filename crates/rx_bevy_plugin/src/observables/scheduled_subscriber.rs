use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, system::Commands};
use rx_bevy::{ObservableOutput, ObserverInput};

use crate::{DebugBound, RxComplete, RxError, RxNext, RxTick, SubscriptionContext};

pub trait ScheduledSubscription: ObservableOutput + DebugBound
where
	Self: Send + Sync,
	Self::Out: Send + Sync,
	Self::OutError: Send + Sync,
{
	/// When set to false, the subscription will not be ticked at all.
	const SCHEDULED: bool = true;

	fn on_tick(&mut self, event: &RxTick, context: SubscriptionContext);

	/// Happens when either the [Subscription] or its relation from [Subscriptions] is removed
	///
	/// > Note that when this runs, this [ScheduledSubscription] instance is already removed
	/// > from the [SubscriptionComponent], not that you would ever try that, since `self` is that.
	fn unsubscribe(&mut self, _context: SubscriptionContext);
}

impl ScheduledSubscription for () {
	fn on_tick(&mut self, _event: &RxTick, _context: SubscriptionContext) {}

	fn unsubscribe(&mut self, _context: SubscriptionContext) {}
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

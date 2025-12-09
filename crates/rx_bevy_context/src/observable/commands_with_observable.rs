use bevy_ecs::system::Commands;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Observable, SchedulerHandle, Subscriber, UpgradeableObserver};

use crate::{EntitySubscription, RxBevyScheduler, SubscriptionComponent};

pub trait CommandsWithObservableExtension {
	fn with_observable<O>(
		&mut self,
		observable: O,
		despawn_scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> CommandsWithObservable<'_, '_, O>
	where
		O: Observable;
}

impl CommandsWithObservableExtension for Commands<'_, '_> {
	fn with_observable<O>(
		&mut self,
		observable: O,
		despawn_scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> CommandsWithObservable<'_, '_, O>
	where
		O: Observable,
	{
		CommandsWithObservable {
			commands: self.reborrow(),
			observable,
			despawn_scheduler,
		}
	}
}

pub trait ObservableWithCommandsExtension
where
	Self: Sized + Observable,
{
	fn with_commands<'w, 's>(
		self,
		commands: Commands<'w, 's>,
		despawn_scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> CommandsWithObservable<'w, 's, Self>;
}

impl<O> ObservableWithCommandsExtension for O
where
	O: Observable,
{
	fn with_commands<'w, 's>(
		self,
		commands: Commands<'w, 's>,
		despawn_scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> CommandsWithObservable<'w, 's, O> {
		CommandsWithObservable {
			commands,
			observable: self,
			despawn_scheduler,
		}
	}
}

#[derive(RxObservable)]
#[rx_out(O::Out)]
#[rx_out_error(O::OutError)]
pub struct CommandsWithObservable<'w, 's, O>
where
	O: Observable,
{
	commands: Commands<'w, 's>,
	despawn_scheduler: SchedulerHandle<RxBevyScheduler>,
	observable: O,
}

impl<'w, 's, O> Observable for CommandsWithObservable<'w, 's, O>
where
	O: Observable,
{
	type Subscription<Destination>
		= EntitySubscription
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let mut subscription_entity = self.commands.spawn_empty();

		let subscription = self.observable.subscribe(destination);

		subscription_entity.insert(SubscriptionComponent::new(
			subscription,
			subscription_entity.id(),
			self.despawn_scheduler.clone(),
		));

		EntitySubscription::new(subscription_entity.id(), self.despawn_scheduler.clone())
	}
}

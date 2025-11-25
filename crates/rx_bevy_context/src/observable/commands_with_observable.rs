use std::marker::PhantomData;

use bevy_ecs::{schedule::ScheduleLabel, system::Commands};
use rx_bevy_common::Clock;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Observable, Subscriber, UpgradeableObserver};

use crate::{
	BevySubscriptionContextProvider, EntitySubscription, ScheduledSubscriptionComponent,
	SubscriptionSchedule,
};

pub trait CommandsWithObservableExtension {
	fn with_observable<O, S, C>(
		&mut self,
		observable: O,
	) -> CommandsWithObservable<'_, '_, O, S, C>
	where
		O: Observable<Context = BevySubscriptionContextProvider>,
		S: ScheduleLabel,
		C: Clock;
}

impl CommandsWithObservableExtension for Commands<'_, '_> {
	fn with_observable<O, S, C>(&mut self, observable: O) -> CommandsWithObservable<'_, '_, O, S, C>
	where
		O: Observable<Context = BevySubscriptionContextProvider>,
		S: ScheduleLabel,
		C: Clock,
	{
		CommandsWithObservable {
			commands: self.reborrow(),
			observable,
			_phantom_data: PhantomData,
		}
	}
}

pub trait ObservableWithCommandsExtension
where
	Self: Sized + Observable<Context = BevySubscriptionContextProvider>,
{
	fn with_commands<'w, 's, S, C>(
		self,
		commands: Commands<'w, 's>,
	) -> CommandsWithObservable<'w, 's, Self, S, C>
	where
		S: ScheduleLabel,
		C: Clock;
}

impl<O> ObservableWithCommandsExtension for O
where
	O: Observable<Context = BevySubscriptionContextProvider>,
{
	fn with_commands<'w, 's, S, C>(
		self,
		commands: Commands<'w, 's>,
	) -> CommandsWithObservable<'w, 's, O, S, C>
	where
		S: ScheduleLabel,
		C: Clock,
	{
		CommandsWithObservable {
			commands,
			observable: self,
			_phantom_data: PhantomData,
		}
	}
}

#[derive(RxObservable)]
#[rx_out(O::Out)]
#[rx_out_error(O::OutError)]
#[rx_context(BevySubscriptionContextProvider)]
pub struct CommandsWithObservable<'w, 's, O, S, C>
where
	O: Observable<Context = BevySubscriptionContextProvider>,
	S: ScheduleLabel,
	C: Clock,
{
	commands: Commands<'w, 's>,
	observable: O,
	_phantom_data: PhantomData<(S, C)>,
}

impl<'w, 's, O, S, C> Observable for CommandsWithObservable<'w, 's, O, S, C>
where
	O: Observable<Context = BevySubscriptionContextProvider>,
	S: ScheduleLabel,
	C: Clock,
{
	type Subscription<Destination>
		= EntitySubscription
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Context as rx_core_traits::SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let subscription_entity = self
			.commands
			.spawn((SubscriptionSchedule::<S, C>::default(),))
			.id();

		let subscription = self.observable.subscribe(destination, context);

		self.commands
			.entity(subscription_entity)
			.insert(ScheduledSubscriptionComponent::new(
				subscription,
				subscription_entity,
			));

		EntitySubscription::new(subscription_entity)
	}
}

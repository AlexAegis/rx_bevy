use std::marker::PhantomData;

use bevy_ecs::system::EntityCommands;
use rx_core_common::{
	Observable, PhantomInvariant, SchedulerHandle, SharedSubscriber, Signal, Subscriber,
	UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;

use crate::{CommandSubscribeExtension, EntitySubscription, RxBevyScheduler};

pub trait EntityCommandsAsObservableExtension {
	/// # `as_observable`
	///
	/// Turn this entity into an observable to access the entire [Observable]
	/// api! Pipe arbitrary operators that are unique to the subscription
	/// you'll create out of this observable!
	///
	/// Be aware that the `Out` and `OutError` generics must match an observable
	/// that is on this entity!
	///
	/// ## What to do with the returned Subscription after `subscribe`
	///
	/// Unlike
	/// [`Commands::subscribe`][crate::CommandSubscribeExtension::subscribe] or
	/// [`EntityCommands::subscribe`][crate::EntityCommandSubscribeExtension::subscribe]
	/// which return an entity for the spawned subscription, that you can just
	/// simply despawn to stop it, the `subscribe` method on an observable
	/// returns a subscription. The subscription returned from this observable
	/// will always be the [EntityCommandsSubscription][crate::EntityCommandsSubscription]
	/// which doesn't own the actual subscription that really contains the
	/// destination and all the subscribers made from the operators you may have
	/// used. Those are still stored in a real subscription entity.
	///
	/// Therefore this subscription can be turned back into an entity using
	/// [`into_entity`][crate::EntityCommandsSubscription::into_entity] to get
	/// the actual subscriptions entity back.
	///
	/// It is advised to hold onto the subscriptions entity, so you know what
	/// to despawn when you want to stop the subscription. But it's not
	/// required to do so.
	///
	/// ## Implementation Details
	///
	/// ### Observable Spawn and Subscribe Race Conditions
	///
	/// > This only concerns cases where you spawn/insert the observable, and
	/// > subscribe to it in the very same system! It will always work, but in
	/// > some cases the subscription will only be created in the next frame's
	/// > [`First`][bevy_app::main_schedule::First] schedule. Read the details
	/// > below to know exactly when that happens and why.
	///
	/// There is a subtle difference between creating a subscription through this
	/// [`EntityCommands::as_observable`][crate::EntityCommandsAsObservableExtension::as_observable]
	/// api and creating one using
	/// [`Commands::subscribe`][crate::CommandSubscribeExtension::subscribe] or
	/// [`EntityCommands::subscribe`][crate::EntityCommandSubscribeExtension::subscribe]
	/// and similar subscribe functions that are directly on
	/// [`Commands`][bevy_ecs::system::Commands] or
	/// [`EntityCommands`][bevy_ecs::system::EntityCommands].
	/// The latters use the same `Commands` instance that you used to create
	/// the observable, granting you a clear order of queued commands. If you
	/// inserted an observable component onto your observable
	/// entity, and only then you issued the subscribe command, you can be
	/// sure that the subscription will be established in the same frame.
	fn as_observable<Out, OutError>(
		&mut self,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> EntityCommandsObservable<'_, Out, OutError>
	where
		Out: Signal + Clone,
		OutError: Signal + Clone;
}

impl EntityCommandsAsObservableExtension for EntityCommands<'_> {
	fn as_observable<Out, OutError>(
		&mut self,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> EntityCommandsObservable<'_, Out, OutError>
	where
		Out: Signal + Clone,
		OutError: Signal + Clone,
	{
		EntityCommandsObservable {
			observable_entity_commands: self.reborrow(),
			scheduler,
			phantom_data: PhantomData,
		}
	}
}

#[derive(RxObservable)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct EntityCommandsObservable<'w, Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	observable_entity_commands: EntityCommands<'w>,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	phantom_data: PhantomInvariant<(Out, OutError)>,
}

impl<'w, Out, OutError> Observable for EntityCommandsObservable<'w, Out, OutError>
where
	Out: Signal,
	OutError: Signal,
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
		let observable_entity = self.observable_entity_commands.id();

		let shared_destination = SharedSubscriber::new(destination.upgrade());

		let subscription_entity = self
			.observable_entity_commands
			.commands()
			.subscribe(observable_entity, shared_destination.clone());

		EntitySubscription::new(
			subscription_entity,
			shared_destination,
			self.scheduler.clone(),
		)
	}
}

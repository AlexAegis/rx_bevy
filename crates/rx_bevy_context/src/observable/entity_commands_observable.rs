use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, schedule::ScheduleLabel, system::EntityCommands};
use rx_bevy_common::Clock;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Observable, SignalBound, Subscriber, SubscriptionContext, UpgradeableObserver,
};

use crate::{EntityCommandSubscribeExtension, EntitySubscription, RxBevyContext};

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
	/// ### Observable spawn and subscribe Race Conditions
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
	fn as_observable<Out, OutError, S, C>(
		&mut self,
	) -> EntityCommandsObservable<Out, OutError, S, C>
	where
		Out: SignalBound + Clone,
		OutError: SignalBound + Clone,
		S: ScheduleLabel,
		C: Clock;
}

impl EntityCommandsAsObservableExtension for EntityCommands<'_> {
	fn as_observable<Out, OutError, S, C>(
		&mut self,
	) -> EntityCommandsObservable<Out, OutError, S, C>
	where
		Out: SignalBound + Clone,
		OutError: SignalBound + Clone,
		S: ScheduleLabel,
		C: Clock,
	{
		self.id().into()
	}
}

impl EntityCommandsAsObservableExtension for Entity {
	fn as_observable<Out, OutError, S, C>(
		&mut self,
	) -> EntityCommandsObservable<Out, OutError, S, C>
	where
		Out: SignalBound + Clone,
		OutError: SignalBound + Clone,
		S: ScheduleLabel,
		C: Clock,
	{
		(*self).into()
	}
}

#[derive(RxObservable)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
#[rx_context(RxBevyContext)]
pub struct EntityCommandsObservable<Out, OutError, S, C>
where
	Out: SignalBound,
	OutError: SignalBound,
	S: ScheduleLabel,
	C: Clock,
{
	observable_entity: Entity,
	phantom_data: PhantomData<(Out, OutError, S, C)>,
}

impl<Out, OutError, S, C> From<Entity> for EntityCommandsObservable<Out, OutError, S, C>
where
	Out: SignalBound,
	OutError: SignalBound,
	S: ScheduleLabel,
	C: Clock,
{
	fn from(entity: Entity) -> Self {
		Self {
			observable_entity: entity,
			phantom_data: PhantomData,
		}
	}
}

impl<Out, OutError, S, C> Observable for EntityCommandsObservable<Out, OutError, S, C>
where
	Out: SignalBound,
	OutError: SignalBound,
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
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		// If the observable was spawned in the same system from where you call
		// subscribe, the command that would insert the Observable onto
		// `self.observable_entity`, has definitely not happend yet.
		// This would be a big problem if not for retry-able subscribe commands!
		let subscription_entity = context
			.deferred_world
			.commands()
			.entity(self.observable_entity)
			.subscribe_destination::<_, S, C>(destination);
		EntitySubscription::new(subscription_entity)
	}
}

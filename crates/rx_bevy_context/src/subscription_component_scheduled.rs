use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	world::DeferredWorld,
};
use rx_core_traits::{
	ObservableSubscription, SubscriptionLike, Teardown, Tick, Tickable, WithSubscriptionContext,
};
use short_type_name::short_type_name;
use stealcell::{StealCell, Stolen};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextParam, BevySubscriptionContextProvider,
	ConsumableSubscriptionNotificationEvent, SubscriptionNotificationEvent,
};

// TODO(bevy-0.18+): This component does not need to be erased, it's only erased to facilitate mass unsubscribe on exit, which currently can't be done using commands as there is no teardown schedule in bevy similar to the startup schedule. https://github.com/AlexAegis/rx_bevy/issues/2 https://github.com/bevyengine/bevy/issues/7067
#[derive(Component)]
#[component(on_insert=scheduled_subscription_add_notification_observer_on_insert, on_remove=scheduled_subscription_unsubscribe_on_remove)]
#[require(Name::new(short_type_name::<Self>()))]
pub struct ScheduledSubscriptionComponent {
	this_entity: Entity,
	// TODO(bevy-0.18+): This "StealCell" won't be necessary once entity world scope lands: https://github.com/AlexAegis/rx_bevy/issues/1 https://github.com/bevyengine/bevy/issues/13128
	subscription: StealCell<
		Box<dyn ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync>,
	>,
}

impl ScheduledSubscriptionComponent {
	pub fn new<Subscription>(subscription: Subscription, this_entity: Entity) -> Self
	where
		Subscription: 'static
			+ ObservableSubscription<Context = BevySubscriptionContextProvider>
			+ Send
			+ Sync,
	{
		Self {
			subscription: StealCell::new(Box::new(subscription)),
			this_entity,
		}
	}

	fn get_subscription(
		&self,
	) -> &dyn ObservableSubscription<Context = BevySubscriptionContextProvider> {
		self.subscription.as_deref()
	}

	fn get_subscription_mut(
		&mut self,
	) -> &mut dyn ObservableSubscription<Context = BevySubscriptionContextProvider> {
		self.subscription.as_deref_mut()
	}

	pub fn steal_subscription(
		&mut self,
	) -> Stolen<
		Box<dyn ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync>,
	> {
		self.subscription.steal()
	}

	pub fn return_stolen_subscription(
		&mut self,
		subscription: Stolen<
			Box<
				dyn ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
			>,
		>,
	) {
		self.subscription.return_stolen(subscription)
	}
}

pub(crate) fn scheduled_subscription_add_notification_observer_on_insert(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) {
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);
	entity_commands.insert(Observer::new(scheduled_subscription_notification_observer));
}

pub(crate) fn scheduled_subscription_notification_observer(
	mut subscription_notification: Trigger<ConsumableSubscriptionNotificationEvent>,
	context_param: BevySubscriptionContextParam,
) -> Result<(), BevyError> {
	let subscription_entity = subscription_notification.target();
	let mut context = context_param.into_context(subscription_entity);

	let mut scheduled_subscription_component =
		context.try_get_component_mut::<ScheduledSubscriptionComponent>(subscription_entity)?;
	let mut stolen_scheduled_subscription = scheduled_subscription_component.steal_subscription();

	let event = subscription_notification.event_mut().consume();

	match event {
		SubscriptionNotificationEvent::Unsubscribe => {
			stolen_scheduled_subscription.unsubscribe(&mut context);
			context
				.deferred_world
				.commands()
				.entity(subscription_entity)
				.despawn();
		}
		SubscriptionNotificationEvent::Tick(tick) => {
			stolen_scheduled_subscription.tick(tick, &mut context);
		}
		SubscriptionNotificationEvent::Add(teardown) => {
			stolen_scheduled_subscription.add_teardown(teardown, &mut context);
		}
	};

	context
		.return_stolen_scheduled_subscription(subscription_entity, stolen_scheduled_subscription)?;

	Ok(())
}

fn scheduled_subscription_unsubscribe_on_remove(
	deferred_world: DeferredWorld,
	hook_context: HookContext,
) {
	println!("scheduled_subscription_unsubscribe_on_remove");
	let context_param: BevySubscriptionContextParam = deferred_world.into();
	let mut context = context_param.into_context(hook_context.entity);

	println!("subscription, unsub on remove?? {}", hook_context.entity);
	let mut stolen_subscription = context
		.steal_scheduled_subscription(hook_context.entity)
		.unwrap();
	stolen_subscription.unsubscribe(&mut context);
	context
		.return_stolen_scheduled_subscription(hook_context.entity, stolen_subscription)
		.unwrap();
}

impl WithSubscriptionContext for ScheduledSubscriptionComponent {
	type Context = BevySubscriptionContextProvider;
}

impl Tickable for ScheduledSubscriptionComponent {
	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		let subscription = self.get_subscription_mut();
		subscription.tick(tick, context);
	}
}

impl SubscriptionLike for ScheduledSubscriptionComponent {
	#[inline]
	fn is_closed(&self) -> bool {
		let subscription = self.get_subscription();
		subscription.is_closed()
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		let subscription = self.get_subscription_mut();
		subscription.unsubscribe(context);
		context
			.deferred_world
			.commands()
			.entity(self.this_entity)
			.try_despawn();
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut BevySubscriptionContext<'_, '_>,
	) {
		let subscription = self.get_subscription_mut();
		subscription.add_teardown(teardown, context);
	}
}

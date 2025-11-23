use std::marker::PhantomData;

use bevy_ecs::resource::Resource;
use rx_bevy_context::{BevySubscriptionContext, BevySubscriptionContextProvider};
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Subscriber, SubscriptionContext, SubscriptionData, SubscriptionLike, Teardown,
	TeardownCollection, Tick, Tickable, WithSubscriptionContext,
};

use crate::observable::ResourceObservableOptions;

#[derive(RxSubscription)]
#[rx_context(BevySubscriptionContextProvider)]
pub struct ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Destination::In, Destination::InError> + Clone + Send + Sync,
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	destination: Destination,
	reader: Reader,
	options: ResourceObservableOptions,
	teardown: SubscriptionData<BevySubscriptionContextProvider>,
	// The `is_resource_added` method doesn't seem to work in this context, so
	// it will be tracked here instead.
	resource_existed_in_the_previous_tick: bool,
	_phantom_data: PhantomData<R>,
}

impl<R, Reader, Destination> ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Destination::In, Destination::InError> + Clone + Send + Sync,
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	pub fn new(
		reader: Reader,
		options: ResourceObservableOptions,
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			reader,
			options,
			destination,
			resource_existed_in_the_previous_tick: context
				.deferred_world
				.get_resource::<R>()
				.is_some(),
			teardown: SubscriptionData::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<R, Reader, Destination> SubscriptionLike for ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Destination::In, Destination::InError> + Clone + Send + Sync,
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	#[inline]
	#[track_caller]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.destination.unsubscribe(context);
			self.teardown.unsubscribe(context);
		}
	}
}

impl<R, Reader, Destination> TeardownCollection for ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Destination::In, Destination::InError> + Clone + Send + Sync,
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	#[track_caller]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.teardown.add_teardown(teardown, context);
		} else {
			teardown.execute(context);
		}
	}
}

impl<R, Reader, Destination> Tickable for ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Destination::In, Destination::InError> + Clone + Send + Sync,
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	#[track_caller]
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		let resource_option = context.deferred_world.get_resource::<R>();
		let is_changed = context.deferred_world.is_resource_changed::<R>();
		let is_added = {
			let resource_exists_this_tick = resource_option.is_some();
			let is_added = !self.resource_existed_in_the_previous_tick && resource_exists_this_tick;
			self.resource_existed_in_the_previous_tick = resource_exists_this_tick;
			is_added
		};

		// is_changed is always true when is_added is true
		let is_changed_condition = self.options.trigger_on_is_changed && is_changed && !is_added;
		let is_added_condition = self.options.trigger_on_is_added && is_added;

		if (is_changed_condition || is_added_condition)
			&& let Some(resource) = resource_option
		{
			let next = (self.reader)(resource);
			match next {
				Ok(next) => self.destination.next(next, context),
				Err(error) => self.destination.error(error, context),
			}
		}

		self.destination.tick(tick, context);
	}
}

impl<R, Reader, Destination> Drop for ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Destination::In, Destination::InError> + Clone + Send + Sync,
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = <<Self as WithSubscriptionContext>::Context as SubscriptionContext>::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}

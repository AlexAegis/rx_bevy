use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use bevy_ecs::{entity::Entity, resource::Resource};
use bevy_platform::collections::HashMap;
use rx_core_traits::ContextProvider;

use crate::{AsyncAllocationId, RxBevyContext};

#[derive(Resource)]
pub struct RxAsyncAllocator {
	inner: Arc<Mutex<AsyncAllocatorState<RxBevyContext>>>,
}

struct AsyncAllocatorState<C>
where
	C: ContextProvider,
{
	allocations: HashMap<AsyncAllocationId, Box<dyn RxAsyncAllocation<C> + Send + Sync>>,
}

pub struct AsyncAllocationCustom<C, T, O, R>
where
	C: ContextProvider,
	O: FnOnce(&mut C::Item<'_>) -> O,
	R: FnOnce(O, &mut C::Item<'_>),
{
	value: Option<T>,
	obtain: O,
	release: R,
	_phantom_data: PhantomData<fn(C) -> C>,
}

pub trait RxAsyncAllocation<C>
where
	C: ContextProvider,
{
	fn allocate(&mut self, context: &mut C::Item<'_>);
	fn deallocate(&mut self, context: &mut C::Item<'_>);
}

pub struct AsyncAllocationEntity {
	entity: Option<Entity>,
}

impl RxAsyncAllocation<RxBevyContext> for AsyncAllocationEntity {
	fn allocate(&mut self, _context: &mut <RxBevyContext as ContextProvider>::Item<'_>) {}

	fn deallocate(&mut self, context: &mut <RxBevyContext as ContextProvider>::Item<'_>) {
		if let Some(entity) = self.entity {
			context
				.deferred_world
				.commands()
				.entity(entity)
				.try_despawn();
		}
	}
}

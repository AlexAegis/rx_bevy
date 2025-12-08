use core::marker::PhantomData;

use bevy_ecs::{resource::Resource, world::Mut};
use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{Observer, ObserverNotification, Signal, UpgradeableObserver};

use crate::{DetachedSubscriber, RxBevyContextItem};

#[derive(RxObserver, Copy, Clone, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_does_not_upgrade_to_observer_subscriber]
pub struct ResourceDestination<In, InError, R, ResourceWriter>
where
	In: Signal,
	InError: Signal,
	R: Resource,
	ResourceWriter: 'static + FnMut(Mut<'_, R>, ObserverNotification<In, InError>) + Send + Sync,
{
	writer: ResourceWriter,
	_phantom_data: PhantomData<(In, InError, R)>,
}

impl<In, InError, R, ResourceWriter> ResourceDestination<In, InError, R, ResourceWriter>
where
	In: Signal,
	InError: Signal,
	R: Resource,
	ResourceWriter: 'static + FnMut(Mut<'_, R>, ObserverNotification<In, InError>) + Send + Sync,
{
	pub fn new(writer: ResourceWriter) -> Self {
		Self {
			writer,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, R, ResourceWriter> UpgradeableObserver
	for ResourceDestination<In, InError, R, ResourceWriter>
where
	In: Signal,
	InError: Signal,
	R: Resource,
	ResourceWriter: 'static + FnMut(Mut<'_, R>, ObserverNotification<In, InError>) + Send + Sync,
{
	type Upgraded = DetachedSubscriber<Self>;

	fn upgrade(self) -> Self::Upgraded {
		DetachedSubscriber::new(self)
	}
}

impl<In, InError, R, ResourceWriter> Observer
	for ResourceDestination<In, InError, R, ResourceWriter>
where
	In: Signal,
	InError: Signal,
	R: Resource,
	ResourceWriter: 'static + FnMut(Mut<'_, R>, ObserverNotification<In, InError>) + Send + Sync,
{
	fn next(&mut self, next: Self::In, context: &mut RxBevyContextItem<'_, '_>) {
		// TODO: Figure out How to do this without a passed in context. maybe the schedulers
		// TODO: tasks could just do it, if the task context is the same, needs an on-schedule return value that could come from that context, to acquire handles from it
		let resource = context.deferred_world.resource_mut::<R>();
		(self.writer)(resource, ObserverNotification::<In, InError>::Next(next));
	}

	fn error(&mut self, error: Self::InError, context: &mut RxBevyContextItem<'_, '_>) {
		let resource = context.deferred_world.resource_mut::<R>();
		(self.writer)(resource, ObserverNotification::<In, InError>::Error(error));
	}

	fn complete(&mut self, context: &mut RxBevyContextItem<'_, '_>) {
		let resource = context.deferred_world.resource_mut::<R>();
		(self.writer)(resource, ObserverNotification::<In, InError>::Complete);
	}
}

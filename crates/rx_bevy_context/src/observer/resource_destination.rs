use core::marker::PhantomData;

use bevy_ecs::{resource::Resource, world::Mut};
use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{Observer, ObserverNotification, SignalBound, UpgradeableObserver};

use crate::{DetachedSubscriber, RxBevyContext, RxBevyContextItem};

#[derive(RxObserver, Copy, Clone, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(RxBevyContext)]
#[rx_does_not_upgrade_to_observer_subscriber]
pub struct ResourceDestination<In, InError, R, ResourceWriter>
where
	In: SignalBound,
	InError: SignalBound,
	R: Resource,
	ResourceWriter: 'static + FnMut(Mut<'_, R>, ObserverNotification<In, InError>) + Send + Sync,
{
	writer: ResourceWriter,
	_phantom_data: PhantomData<(In, InError, R)>,
}

impl<In, InError, R, ResourceWriter> ResourceDestination<In, InError, R, ResourceWriter>
where
	In: SignalBound,
	InError: SignalBound,
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
	In: SignalBound,
	InError: SignalBound,
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
	In: SignalBound,
	InError: SignalBound,
	R: Resource,
	ResourceWriter: 'static + FnMut(Mut<'_, R>, ObserverNotification<In, InError>) + Send + Sync,
{
	fn next(&mut self, next: Self::In, context: &mut RxBevyContextItem<'_, '_>) {
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

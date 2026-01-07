use core::marker::PhantomData;
use std::sync::{Arc, Mutex};

use bevy_ecs::{resource::Resource, world::Mut};
use rx_core_common::{
	Observer, ObserverNotification, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension,
	Signal, UpgradeableObserver, WorkCancellationId,
};
use rx_core_macro_observer_derive::RxObserver;

use crate::{DetachedSubscriber, RxBevyContext};

#[derive(RxObserver, Clone, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_does_not_upgrade_to_observer_subscriber]
pub struct ResourceDestination<In, InError, R, ResourceWriter, S>
where
	In: Signal,
	InError: Signal,
	R: Resource,
	ResourceWriter: 'static + FnMut(Mut<'_, R>, ObserverNotification<In, InError>) + Send + Sync,
	S: Scheduler<WorkContextProvider = RxBevyContext>,
{
	writer: Arc<Mutex<ResourceWriter>>,
	owner_id: WorkCancellationId,
	scheduler: SchedulerHandle<S>,
	_phantom_data: PhantomData<(In, InError, R)>,
}

impl<In, InError, R, ResourceWriter, S> ResourceDestination<In, InError, R, ResourceWriter, S>
where
	In: Signal,
	InError: Signal,
	R: Resource,
	ResourceWriter: 'static + FnMut(Mut<'_, R>, ObserverNotification<In, InError>) + Send + Sync,
	S: Scheduler<WorkContextProvider = RxBevyContext>,
{
	pub fn new(writer: ResourceWriter, scheduler: SchedulerHandle<S>) -> Self {
		let owner_id = scheduler.lock().generate_cancellation_id();
		Self {
			writer: Arc::new(Mutex::new(writer)),
			owner_id,
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, R, ResourceWriter, S> UpgradeableObserver
	for ResourceDestination<In, InError, R, ResourceWriter, S>
where
	In: Signal,
	InError: Signal,
	R: Resource,
	ResourceWriter: 'static + FnMut(Mut<'_, R>, ObserverNotification<In, InError>) + Send + Sync,
	S: Scheduler<WorkContextProvider = RxBevyContext>,
{
	type Upgraded = DetachedSubscriber<Self>;

	fn upgrade(self) -> Self::Upgraded {
		DetachedSubscriber::new(self)
	}
}

impl<In, InError, R, ResourceWriter, S> Observer
	for ResourceDestination<In, InError, R, ResourceWriter, S>
where
	In: Signal,
	InError: Signal,
	R: Resource,
	ResourceWriter: 'static + FnMut(Mut<'_, R>, ObserverNotification<In, InError>) + Send + Sync,
	S: Scheduler<WorkContextProvider = RxBevyContext>,
{
	fn next(&mut self, next: Self::In) {
		let writer = self.writer.clone();
		self.scheduler.lock().schedule_immediate_work(
			move |_, context| {
				if let Ok(mut writer) = writer.lock() {
					let resource = context.deferred_world.resource_mut::<R>();
					(writer)(resource, ObserverNotification::<In, InError>::Next(next));
				}
			},
			self.owner_id,
		);
	}

	fn error(&mut self, error: Self::InError) {
		let writer = self.writer.clone();

		self.scheduler.lock().schedule_immediate_work(
			move |_, context| {
				if let Ok(mut writer) = writer.lock() {
					let resource = context.deferred_world.resource_mut::<R>();
					(writer)(resource, ObserverNotification::<In, InError>::Error(error));
				}
			},
			self.owner_id,
		);
	}

	fn complete(&mut self) {
		let writer = self.writer.clone();
		self.scheduler.lock().schedule_immediate_work(
			move |_, context| {
				if let Ok(mut writer) = writer.lock() {
					let resource = context.deferred_world.resource_mut::<R>();
					(writer)(resource, ObserverNotification::<In, InError>::Complete);
				}
			},
			self.owner_id,
		);
	}
}

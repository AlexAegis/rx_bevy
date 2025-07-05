use crate::{ObservableComponent, RxBufferedSubscriber, SubscriptionComponent};
use crate::{RxNext, SubscriptionComponentLike};
use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use rx_bevy::Observer;
use rx_bevy::prelude::*;
use smallvec::SmallVec;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Component, Debug)]
#[component(on_insert=on_subject_insert::<In, InError>)]
pub struct SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	subscribers: SmallVec<[RxBufferedSubscriber<In, InError>; 2]>,
	_phantom_data: PhantomData<(In, InError)>,
}

fn on_subject_insert<In, InError>(mut world: DeferredWorld, hook_context: HookContext)
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	let mut commands = world.commands();
	let mut entity = commands.entity(hook_context.entity);
	// TODO(bevy-0.17): the created observer should be tied to this subject
	entity.observe(
		move |trigger: Trigger<RxNext<In>>,
		      mut subject_query: Query<&mut SubjectComponent<In, InError>>| {
			if let Ok(mut subject_component) = subject_query.get_mut(hook_context.entity) {
				subject_component.next(trigger.event().0.clone());
			}
		},
	);
}

impl<In, InError> SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	pub fn new() -> Self {
		Self {
			subscribers: SmallVec::new(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ObservableComponent for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	fn component_subscribe(
		&mut self,
		subscriber: RxBufferedSubscriber<Self::Out, Self::OutError>,
	) -> Option<SubscriptionComponent<Self::Out, Self::OutError>> {
		self.subscribers.push(subscriber);
		None
	}
}

impl<In, InError> SubscriptionComponentLike for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	fn is_closed(&self) -> bool {
		self.subscribers.iter().all(|s| s.is_closed())
	}

	fn unsubscribe(&mut self) {
		for mut subscriber in self.subscribers.drain(..) {
			subscriber.unsubscribe();
		}
	}

	fn flush(&mut self, commands: &mut Commands) -> bool {
		let mut flushed_something = true;
		for subscriber in self.subscribers.iter_mut() {
			flushed_something = flushed_something && subscriber.flush(commands);
		}
		flushed_something
	}
}

impl<In, InError> Observer for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	fn next(&mut self, next: Self::In) {
		for subscriber in self.subscribers.iter_mut() {
			subscriber.next(next.clone());
		}
	}

	fn error(&mut self, error: Self::InError) {
		for subscriber in self.subscribers.iter_mut() {
			subscriber.error(error.clone());
		}
	}

	fn complete(&mut self) {
		for subscriber in self.subscribers.iter_mut() {
			subscriber.complete();
		}
	}
}

impl<In, InError> ObserverInput for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> ObservableOutput for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	type Out = In;
	type OutError = InError;
}

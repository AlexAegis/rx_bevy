use bevy_ecs::{
	component::{Component, HookContext},
	world::DeferredWorld,
};
use rx_core_common::{
	Observable, RxObserver, SubjectLike, Subscriber, SubscriptionLike, UpgradeableObserver,
};
use rx_core_macro_subject_derive::RxSubject;

use crate::{
	ObservableOutputs, ObservableSubscriptions, SignalObserverSatelliteBundle,
	SubscribeEventObserverSatelliteBundle, SubscribeObserverRef,
};

/// # [SubjectComponent]
///
/// On top of acting like an [ObservableComponent][crate::ObservableComponent]
/// it also automatically pushes all observed [RxSignal][crate::RxSignal]
/// events into the subject contained in it. Can be used to broadcast events
/// from the ECS.
///
/// It can act as the destination of a subscription!
///
/// ## See Also
///
/// - [`PublishSubject`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_publish) -
///   The basic multicasting primitive, signals pushed into it will be received
///   by all active subscribers!
/// - [`BehaviorSubject`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_behavior) -
///   A subject that always has a stored value that is instantly replayed for
///   new subscribers.
/// - [`ReplaySubject`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_replay) -
///   A subject that can replay the `n` last observed values back to new
///   subscribers.
/// - [`AsyncSubject`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_async) -
///   A subject that only emits one value, once it's completed. All observed
///   values are reduced into a single value until a completion is triggered.
/// - [`ProvenanceSubject`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_provenance) -
///   A BehaviorSubject that tracks where the stores an additional "Provenance"
///   value signaling where the value originated from. It offers filters to
///   only receive events from certain origins, or from all.
#[derive(Component, RxSubject)]
#[rx_in(Subject::In)]
#[rx_in_error(Subject::InError)]
#[rx_out(Subject::Out)]
#[rx_out_error(Subject::OutError)]
#[rx_delegate_subscription_like_to_destination]
#[component(on_insert=subject_on_insert::<Subject>, on_remove=subject_on_remove::<Subject>)]
#[require(ObservableSubscriptions::<Subject>, ObservableOutputs::<Subject::Out, Subject::OutError>)]
pub struct SubjectComponent<Subject>
where
	Subject: SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	#[destination]
	subject: Subject,
}

impl<Subject> SubjectComponent<Subject>
where
	Subject: SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	pub fn new(subject: Subject) -> Self {
		Self { subject }
	}
}

impl<Subject> Observable for SubjectComponent<Subject>
where
	Subject: SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	type Subscription<Destination>
		= Subject::Subscription<Destination>
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
		self.subject.subscribe(destination)
	}
}

impl<Subject> RxObserver for SubjectComponent<Subject>
where
	Subject: SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.subject.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.subject.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.subject.complete();
	}
}

fn subject_on_insert<Subject>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	Subject: 'static + SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	let mut commands = deferred_world.commands();
	commands.spawn(SubscribeEventObserverSatelliteBundle::<Subject>::new::<
		SubjectComponent<Subject>,
	>(hook_context.entity));

	commands.spawn(SignalObserverSatelliteBundle::<Subject>::new::<
		SubjectComponent<Subject>,
	>(hook_context.entity));
}

/// Remove related components along with the subject
fn subject_on_remove<Subject>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	Subject: 'static + SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	deferred_world
		.commands()
		.entity(hook_context.entity)
		.remove::<ObservableSubscriptions<Subject>>()
		.remove::<SubscribeObserverRef<Subject>>();

	let mut subject_component = deferred_world
		.get_mut::<SubjectComponent<Subject>>(hook_context.entity)
		.unwrap();

	subject_component.unsubscribe();
}

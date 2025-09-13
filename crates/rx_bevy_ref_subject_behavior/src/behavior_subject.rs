use std::{cell::RefCell, rc::Rc};

use rx_bevy_core::{
	DropSubscription, Observable, ObservableOutput, Observer, ObserverInput, SubscriptionLike,
	Tick, UpgradeableObserver,
};
use rx_bevy_ref_subject::Subject;

/// A BehaviorSubject always contains a value, and immediately emits it
/// on subscription.
#[derive(Clone)]
pub struct BehaviorSubject<In, Error = ()>
where
	In: 'static,
	Error: 'static,
{
	subject: Subject<In, Error>,
	/// RefCell so even cloned subjects retain the same current value across clones
	value: Rc<RefCell<In>>,
}

impl<T, Error> BehaviorSubject<T, Error>
where
	T: Clone,
	Error: Clone,
{
	pub fn new(value: T) -> Self {
		Self {
			subject: Subject::default(),
			value: Rc::new(RefCell::new(value)),
		}
	}

	/// Returns a clone of the currently stored value
	/// In case you want to access the current value, prefer using a
	/// subscription though to keep your code reactive, only use this when it's
	/// absolutely necessary.
	pub fn value(&self) -> T {
		self.value.borrow().clone()
	}
}

impl<T, Error> ObserverInput for BehaviorSubject<T, Error>
where
	T: Clone,
	Error: Clone,
{
	type In = T;
	type InError = Error;
}

impl<T, Error> Observer for BehaviorSubject<T, Error>
where
	T: Clone,
	Error: Clone,
{
	fn next(&mut self, next: T) {
		let n = next.clone();
		self.value.replace(next);
		self.subject.next(n);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.subject.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.subject.complete();
	}

	#[inline]
	fn tick(&mut self, tick: Tick) {
		self.subject.tick(tick);
	}
}

impl<T, Error> ObservableOutput for BehaviorSubject<T, Error>
where
	T: Clone + 'static,
	Error: Clone + 'static,
{
	type Out = T;
	type OutError = Error;
}

impl<T, Error> Observable for BehaviorSubject<T, Error>
where
	T: Clone + 'static,
	Error: Clone + 'static,
{
	type Subscription = DropSubscription<Self::Context>;
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> DropSubscription {
		let mut subscriber = destination.upgrade();

		subscriber.next(self.value.borrow().clone());
		self.subject.subscribe(subscriber)
	}
}

impl<T, Error> SubscriptionLike for BehaviorSubject<T, Error>
where
	T: 'static + Clone,
	Error: 'static + Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.subject.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.subject.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: impl Into<Teardown>) {
		self.subject.add(subscription);
	}
}

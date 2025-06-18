use std::{cell::RefCell, rc::Rc};

use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, ObserverInput, Subscription,
	prelude::ObserverSubscriber,
};
use rx_bevy_subject::{MulticastOperator, MulticastOuterSubscriber, Subject};

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

	fn error(&mut self, error: Self::InError) {
		self.subject.error(error);
	}

	fn complete(&mut self) {
		self.subject.complete();
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
	type Subscriber<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>> =
		MulticastOuterSubscriber<ObserverSubscriber<Destination>>;

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		mut observer: Destination,
	) -> Subscription<Self::Subscriber<Destination>> {
		observer.next(self.value.borrow().clone());
		self.subject.subscribe(observer)
	}
}

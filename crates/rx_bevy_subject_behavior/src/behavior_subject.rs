use std::{cell::RefCell, rc::Rc};

use rx_bevy_observable::{Observable, Observer};
use rx_bevy_subject::{Subject, SubjectSubscription};

/// A BehaviorSubject always contains a value, and immediately emits it
/// on subscription.
#[derive(Clone)]
pub struct BehaviorSubject<T, Error = ()> {
	subject: Subject<T, Error>,
	/// RefCell so even cloned subjects retain the same current value across clones
	value: Rc<RefCell<T>>,
}

impl<T, Error> BehaviorSubject<T, Error>
where
	T: Clone,
	Error: Clone,
{
	pub fn new(value: T) -> Self {
		Self {
			subject: Subject::new(),
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

impl<T, Error> Observer for BehaviorSubject<T, Error>
where
	T: Clone,
	Error: Clone,
{
	type In = T;
	type Error = Error;

	fn next(&mut self, next: T) {
		let n = next.clone();
		self.value.replace(next);
		self.subject.next(n);
	}

	fn error(&mut self, error: Self::Error) {
		self.subject.error(error);
	}

	fn complete(&mut self) {
		self.subject.complete();
	}
}

impl<T, Error> Observable for BehaviorSubject<T, Error>
where
	T: Clone,
	Error: Clone,
{
	type Out = T;
	type Error = Error;
	type Subscription = SubjectSubscription<T, Error>;

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::Error>>(
		&mut self,
		mut observer: Destination,
	) -> Self::Subscription {
		observer.next(self.value.borrow().clone());
		self.subject.subscribe(observer)
	}
}

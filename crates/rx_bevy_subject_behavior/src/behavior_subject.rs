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

	fn on_push(&mut self, next: T) {
		let n = next.clone();
		self.value.replace(next);
		self.subject.on_push(n);
	}

	fn on_error(&mut self, error: Self::Error) {
		self.subject.on_error(error);
	}

	fn on_complete(&mut self) {
		self.subject.on_complete();
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

	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::Error>>(
		&mut self,
		mut observer: Destination,
	) -> Self::Subscription {
		observer.on_push(self.value.borrow().clone());
		self.subject.subscribe(observer)
	}
}

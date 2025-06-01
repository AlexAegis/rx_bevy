use std::{cell::RefCell, rc::Rc};

use rx_bevy_observable::{Observable, Observer};
use rx_bevy_subject::{Subject, SubjectSubscription};

/// A BehaviorSubject always contains a value, and immediately emits it
/// on subscription.
#[derive(Clone)]
pub struct BehaviorSubject<T> {
	subject: Subject<T>,
	/// Refcell so even cloned subjects retain the same current value across clones
	value: Rc<RefCell<T>>,
}

impl<T> BehaviorSubject<T>
where
	T: Clone,
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

impl<T> Observer for BehaviorSubject<T>
where
	T: Clone,
{
	type In = T;
	fn on_push(&mut self, next: T) {
		let n = next.clone();
		self.value.replace(next);
		self.subject.on_push(n);
	}
}

impl<T> Observable for BehaviorSubject<T>
where
	T: Clone,
{
	type Out = T;
	type Subscription = SubjectSubscription<T>;

	fn subscribe<Destination: 'static + Observer<In = Self::Out>>(
		&mut self,
		mut observer: Destination,
	) -> Self::Subscription {
		observer.on_push(self.value.borrow().clone());
		self.subject.subscribe(observer)
	}
}

use std::{cell::RefCell, rc::Rc};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rx_bevy_observable::{Observable, Observer};
use rx_bevy_subject::{Subject, SubjectSubscription};

/// A ReplaySubject - unlike a BehaviorSubject - doesn't always contain a value,
/// but if it does, it immediately returns the last `N` of them upon subscription.
#[derive(Clone)]
pub struct ReplaySubject<T, const CAPACITY: usize> {
	subject: Subject<T>,
	/// Refcell so even cloned subjects retain the same current value across clones
	values: Rc<RefCell<ConstGenericRingBuffer<T, CAPACITY>>>,
}

impl<T, const CAPACITY: usize> ReplaySubject<T, CAPACITY>
where
	T: Clone,
{
	pub fn new() -> Self {
		Self {
			subject: Subject::new(),
			values: Rc::new(RefCell::new(ConstGenericRingBuffer::new())),
		}
	}
}

impl<T, const CAPACITY: usize> Observer for ReplaySubject<T, CAPACITY>
where
	T: Clone,
{
	type In = T;
	fn on_push(&mut self, next: T) {
		self.values.borrow_mut().push(next.clone());
		self.subject.on_push(next);
	}
}

impl<T, const CAPACITY: usize> Observable for ReplaySubject<T, CAPACITY>
where
	T: Clone,
{
	type Out = T;
	type Subscription = SubjectSubscription<T>;

	fn subscribe<Destination: 'static + Observer<In = Self::Out>>(
		&mut self,
		mut observer: Destination,
	) -> Self::Subscription {
		for value in self.values.borrow().iter() {
			observer.on_push(value.clone());
		}

		self.subject.subscribe(observer)
	}
}

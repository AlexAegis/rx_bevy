use std::{cell::RefCell, rc::Rc};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rx_bevy_observable::{Observable, Observer};
use rx_bevy_subject::{Subject, SubjectSubscription};

/// A ReplaySubject - unlike a BehaviorSubject - doesn't always contain a value,
/// but if it does, it immediately returns the last `N` of them upon subscription.
#[derive(Clone)]
pub struct ReplaySubject<const CAPACITY: usize, T, Error = ()> {
	subject: Subject<T, Error>,
	/// Refcell so even cloned subjects retain the same current value across clones
	values: Rc<RefCell<ConstGenericRingBuffer<T, CAPACITY>>>,
}

impl<const CAPACITY: usize, T, Error> ReplaySubject<CAPACITY, T, Error>
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

impl<const CAPACITY: usize, T, Error> Observer for ReplaySubject<CAPACITY, T, Error>
where
	T: Clone,
	Error: Clone,
{
	type In = T;
	type Error = Error;

	fn on_push(&mut self, next: T) {
		self.values.borrow_mut().push(next.clone());
		self.subject.on_push(next);
	}

	fn on_error(&mut self, error: Self::Error) {
		self.subject.on_error(error);
	}

	fn on_complete(&mut self) {
		self.subject.on_complete();
	}
}

impl<const CAPACITY: usize, T, Error> Observable for ReplaySubject<CAPACITY, T, Error>
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
		for value in self.values.borrow().iter() {
			observer.on_push(value.clone());
		}

		self.subject.subscribe(observer)
	}
}

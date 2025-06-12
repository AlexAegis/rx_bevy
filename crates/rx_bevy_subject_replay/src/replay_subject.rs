use std::{cell::RefCell, rc::Rc};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rx_bevy_observable::{Observable, ObservableOutput, Observer};
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

	fn next(&mut self, next: T) {
		self.values.borrow_mut().push(next.clone());
		self.subject.next(next);
	}

	fn error(&mut self, error: Self::Error) {
		self.subject.error(error);
	}

	fn complete(&mut self) {
		self.subject.complete();
	}
}

impl<const CAPACITY: usize, T, Error> ObservableOutput for ReplaySubject<CAPACITY, T, Error>
where
	T: Clone + 'static,
	Error: Clone + 'static,
{
	type Out = T;
	type OutError = Error;
}

impl<const CAPACITY: usize, T, Error> Observable for ReplaySubject<CAPACITY, T, Error>
where
	T: Clone + 'static,
	Error: Clone + 'static,
{
	type Subscription = SubjectSubscription<T, Error>;

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		mut observer: Destination,
	) -> Self::Subscription {
		for value in self.values.borrow().iter() {
			observer.next(value.clone());
		}

		self.subject.subscribe(observer)
	}
}

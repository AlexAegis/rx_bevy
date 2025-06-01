use std::{
	cell::RefCell,
	marker::PhantomData,
	rc::{Rc, Weak},
};

use rx_bevy_observable::{DynObserverConnector, Observable, Observer, Subscription};

use crate::MulticastObserver;

pub struct SubjectConnector<T> {
	phantom_data: PhantomData<T>,
}

impl<T> SubjectConnector<T> {
	pub fn new() -> Self {
		Self {
			phantom_data: PhantomData,
		}
	}
}

impl<T> DynObserverConnector for SubjectConnector<T> {
	type In = T;
	type Out = T;

	fn push_forward(&mut self, next: Self::In, destination: &mut dyn Observer<In = Self::Out>) {
		destination.on_push(next);
	}
}

pub struct SubjectSubscription<T> {
	key: usize,
	subject_ref: Weak<RefCell<MulticastObserver<SubjectConnector<T>>>>,
}

impl<T> Subscription for SubjectSubscription<T> {
	fn unsubscribe(&mut self) {
		if let Some(subject) = self.subject_ref.upgrade() {
			subject.borrow_mut().destination.remove(self.key);
		}
	}

	fn is_closed(&self) -> bool {
		if let Some(subject) = self.subject_ref.upgrade() {
			!subject.borrow().destination.contains(self.key)
		} else {
			true
		}
	}
}

pub struct Subject<T> {
	destinations: Rc<RefCell<MulticastObserver<SubjectConnector<T>>>>,
}

impl<T> Subject<T> {
	pub fn new() -> Self {
		Self {
			destinations: Rc::new(RefCell::new(
				MulticastObserver::new(SubjectConnector::new()),
			)),
		}
	}
}

impl<T> Observable for Subject<T> {
	type Out = T;

	type Subscription = SubjectSubscription<T>;

	fn subscribe<Destination: 'static + Observer<In = Self::Out>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let key = self.destinations.borrow_mut().add_destination(destination);

		SubjectSubscription {
			key,
			subject_ref: Rc::downgrade(&self.destinations),
		}
	}
}

impl<T> Observer for Subject<T>
where
	T: Clone,
{
	type In = T;

	fn on_push(&mut self, next: Self::In) {
		self.destinations.borrow_mut().on_push(next);
	}
}

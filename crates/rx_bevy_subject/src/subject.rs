use std::{
	cell::RefCell,
	marker::PhantomData,
	rc::{Rc, Weak},
};

use rx_bevy_observable::{DynForwarder, Observable, Observer, Subscription};

use crate::MulticastObserver;

pub struct SubjectConnector<T, Error> {
	phantom_data: PhantomData<(T, Error)>,
}

impl<T, Error> SubjectConnector<T, Error> {
	pub fn new() -> Self {
		Self {
			phantom_data: PhantomData,
		}
	}
}

impl<T, Error> DynForwarder for SubjectConnector<T, Error> {
	type In = T;
	type Out = T;
	type InError = Error;
	type OutError = Error;

	#[inline]
	fn next_forward(
		&mut self,
		next: Self::In,
		destination: &mut dyn Observer<In = Self::Out, Error = Self::OutError>,
	) {
		destination.next(next);
	}

	#[inline]
	fn error_forward(
		&mut self,
		error: Self::InError,
		destination: &mut dyn Observer<In = Self::Out, Error = Self::OutError>,
	) {
		destination.error(error);
	}
}

pub struct SubjectSubscription<T, Error> {
	key: usize,
	subject_ref: Weak<RefCell<MulticastObserver<SubjectConnector<T, Error>>>>,
}

impl<T, Error> Subscription for SubjectSubscription<T, Error> {
	fn unsubscribe(&mut self) {
		if let Some(subject) = self.subject_ref.upgrade() {
			subject.borrow_mut().destination.remove(self.key);
		}
	}

	fn is_closed(&self) -> bool {
		if let Some(subject_refcell) = self.subject_ref.upgrade() {
			let subject = subject_refcell.borrow();
			subject.closed || !subject.destination.contains(self.key)
		} else {
			true
		}
	}
}

/// A Subject is a shared multicast observer, can be used for broadcasting
/// a clone of it still has the same set of subscribers, and is needed if you
/// want to make multiple pipes out of the same subject
pub struct Subject<T, Error = ()> {
	destinations: Rc<RefCell<MulticastObserver<SubjectConnector<T, Error>>>>,
}

impl<T, Error> Clone for Subject<T, Error> {
	/// Cloning a subject keeps all existing destinations
	fn clone(&self) -> Self {
		Self {
			destinations: self.destinations.clone(),
		}
	}
}

impl<T, Error> Subject<T, Error> {
	pub fn new() -> Self {
		Self {
			destinations: Rc::new(RefCell::new(
				MulticastObserver::new(SubjectConnector::new()),
			)),
		}
	}
}

impl<T, Error> Observable for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	type Out = T;
	type Error = Error;

	type Subscription = SubjectSubscription<T, Error>;

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::Error>>(
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

impl<T, Error> Observer for Subject<T, Error>
where
	T: Clone,
	Error: Clone,
{
	type In = T;
	type Error = Error;

	fn next(&mut self, next: Self::In) {
		self.destinations.borrow_mut().next(next);
	}

	fn error(&mut self, error: Self::Error) {
		self.destinations.borrow_mut().error(error);
	}

	fn complete(&mut self) {
		// TODO: Check what a subject actually does on complete
		self.destinations.borrow_mut().complete();
	}
}

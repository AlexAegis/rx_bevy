use std::{
	cell::RefCell,
	marker::PhantomData,
	rc::{Rc, Weak},
};

use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, ObserverInput, Subscription, forwarders::DynForwarder,
	subscribers::ObserverSubscriber,
};

use crate::MulticastSubscriber;

pub struct SubjectConnector<T, Error> {
	phantom_data: PhantomData<(T, Error)>,
}

impl<T, Error> Default for SubjectConnector<T, Error> {
	fn default() -> Self {
		Self {
			phantom_data: PhantomData,
		}
	}
}

impl<T, Error> ObservableOutput for SubjectConnector<T, Error>
where
	T: 'static,
	Error: 'static,
{
	type Out = T;
	type OutError = Error;
}

impl<T, Error> ObserverInput for SubjectConnector<T, Error>
where
	T: 'static,
	Error: 'static,
{
	type In = T;
	type InError = Error;
}

impl<T, Error> DynForwarder for SubjectConnector<T, Error>
where
	T: 'static,
	Error: 'static,
{
	#[inline]
	fn next_forward(
		&mut self,
		next: Self::In,
		destination: &mut dyn Observer<In = Self::Out, InError = Self::OutError>,
	) {
		destination.next(next);
	}

	#[inline]
	fn error_forward(
		&mut self,
		error: Self::InError,
		destination: &mut dyn Observer<In = Self::Out, InError = Self::OutError>,
	) {
		destination.error(error);
	}
}

pub struct SubjectSubscription<T, Error>
where
	T: 'static,
	Error: 'static,
{
	key: usize,
	subject_ref: Weak<RefCell<MulticastSubscriber<SubjectConnector<T, Error>>>>,
}

impl<T, Error> Subscription for SubjectSubscription<T, Error> {
	fn unsubscribe(&mut self) {
		if let Some(subject_refcell) = self.subject_ref.upgrade() {
			let mut subject = subject_refcell.borrow_mut();
			if let Some(destination) = subject.destination.get_mut(self.key) {
				destination.unsubscribe();
			}
			subject.destination.remove(self.key);
		}
	}

	fn is_closed(&self) -> bool {
		if let Some(subject_refcell) = self.subject_ref.upgrade() {
			let subject = subject_refcell.borrow();

			subject
				.destination
				.get(self.key)
				.map(|destination| destination.is_closed())
				.unwrap_or(subject.closed || !subject.destination.contains(self.key))
		} else {
			true
		}
	}
}

/// A Subject is a shared multicast observer, can be used for broadcasting
/// a clone of it still has the same set of subscribers, and is needed if you
/// want to make multiple pipes out of the same subject
pub struct Subject<T, Error = ()>
where
	T: 'static,
	Error: 'static,
{
	destinations: Rc<RefCell<MulticastSubscriber<SubjectConnector<T, Error>>>>,
}

impl<T, Error> Clone for Subject<T, Error> {
	/// Cloning a subject keeps all existing destinations
	fn clone(&self) -> Self {
		Self {
			destinations: self.destinations.clone(),
		}
	}
}

impl<T, Error> Default for Subject<T, Error> {
	fn default() -> Self {
		Self {
			destinations: Rc::new(RefCell::new(MulticastSubscriber::new(
				SubjectConnector::default(),
			))),
		}
	}
}

impl<T, Error> ObservableOutput for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	type Out = T;
	type OutError = Error;
}

impl<T, Error> Observable for Subject<T, Error>
where
	T: 'static,
	Error: 'static,
{
	type Subscription = SubjectSubscription<T, Error>;

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let closable_destination = ObserverSubscriber::new(destination);
		let key = self
			.destinations
			.borrow_mut()
			.add_destination(closable_destination);

		SubjectSubscription {
			key,
			subject_ref: Rc::downgrade(&self.destinations),
		}
	}
}

impl<T, Error> ObserverInput for Subject<T, Error>
where
	T: 'static + Clone,
	Error: 'static + Clone,
{
	type In = T;
	type InError = Error;
}

impl<T, Error> Observer for Subject<T, Error>
where
	T: 'static + Clone,
	Error: 'static + Clone,
{
	fn next(&mut self, next: Self::In) {
		self.destinations.borrow_mut().next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.destinations.borrow_mut().error(error);
	}

	fn complete(&mut self) {
		// TODO: Check what a subject actually does on complete
		self.destinations.borrow_mut().complete();
	}
}

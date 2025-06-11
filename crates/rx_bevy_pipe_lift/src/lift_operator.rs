use std::marker::PhantomData;

use rx_bevy_observable::{LiftedSubscriber, LiftingForwarder, Observable, Observer};
use rx_bevy_operator::LiftingOperator;

pub struct LiftOperator<In, InError, OutObservable, Lifter, ErrorLifter> {
	pub lifter: Lifter,
	/// Defines how incoming errors should be converted for listeners of the downstream, lifted observable. If none, all errors will be ignored
	pub error_lifter: ErrorLifter,
	pub _phantom_data: PhantomData<(In, InError, OutObservable)>,
}

impl<In, InError, OutObservable, Lifter, ErrorLifter> LiftingOperator
	for LiftOperator<In, InError, OutObservable, Lifter, ErrorLifter>
where
	Lifter: Clone + Fn(In) -> OutObservable,
	ErrorLifter: Clone + Fn(InError) -> Option<<OutObservable as Observable>::Error>,
	OutObservable: Observable,
{
	type Fw = LiftForwarder<In, InError, OutObservable, Lifter, ErrorLifter>;

	fn lifted_operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self::Fw as LiftingForwarder>::OutObservable,
				Error = <<Self::Fw as LiftingForwarder>::OutObservable as Observable>::Error,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> LiftedSubscriber<Self::Fw, Destination> {
		LiftedSubscriber::new(
			destination,
			LiftForwarder::new(self.lifter.clone(), self.error_lifter.clone()),
		)
	}
}

pub struct LiftForwarder<In, InError, OutObservable, Lifter, ErrorLifter> {
	pub lifter: Lifter,
	pub error_lifter: ErrorLifter,
	pub index: u32,
	pub _phantom_data: PhantomData<(In, InError, OutObservable)>,
}

impl<In, InError, Out, Lifter, ErrorLifter> LiftForwarder<In, InError, Out, Lifter, ErrorLifter> {
	pub fn new(lifter: Lifter, error_lifter: ErrorLifter) -> Self {
		Self {
			lifter,
			error_lifter,
			index: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OutObservable, Lifter, ErrorLifter> LiftingForwarder
	for LiftForwarder<In, InError, OutObservable, Lifter, ErrorLifter>
where
	Lifter: Fn(In) -> OutObservable,
	ErrorLifter: Clone + Fn(InError) -> Option<<OutObservable as Observable>::Error>,
	OutObservable: Observable,
{
	type In = In;
	type OutObservable = OutObservable;
	type InError = InError;

	#[inline]
	fn next_forward<Destination: Observer<In = OutObservable>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		let lifted = (self.lifter)(next);
		self.index += 1;
		destination.next(lifted);
	}

	#[inline]
	fn error_forward<
		Destination: Observer<In = Self::OutObservable, Error = <Self::OutObservable as Observable>::Error>,
	>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		if let Some(lifted_error) = (self.error_lifter)(error) {
			destination.error(lifted_error);
		}
	}
}

impl<In, InError, Out, Lifter, ErrorLifter> LiftOperator<In, InError, Out, Lifter, ErrorLifter> {
	pub fn new(lifter: Lifter, error_lifter: ErrorLifter) -> Self {
		Self {
			lifter,
			error_lifter,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Out, Lifter, ErrorLifter> Clone
	for LiftOperator<In, InError, Out, Lifter, ErrorLifter>
where
	Lifter: Clone,
	ErrorLifter: Clone,
{
	fn clone(&self) -> Self {
		Self {
			lifter: self.lifter.clone(),
			error_lifter: self.error_lifter.clone(),
			_phantom_data: PhantomData,
		}
	}
}

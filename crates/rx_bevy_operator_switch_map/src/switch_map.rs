use std::marker::PhantomData;

use rx_bevy_observable::{
	Forwarder, LiftedSubscriber, LiftingForwarder, Observable, Observer, Subscriber, Subscription,
};
use rx_bevy_operator::{LiftingOperator, Operator};

pub struct SwitchMapOperator<In, InError, InnerObservable, Switcher> {
	pub switcher: Switcher,
	pub _phantom_data: PhantomData<(In, InError, InnerObservable)>,
}

impl<In, InError, OutObservable, Switcher> SwitchMapOperator<In, InError, OutObservable, Switcher> {
	pub fn new(switcher: Switcher) -> Self {
		Self {
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OutObservable, Switcher> Clone
	for SwitchMapOperator<In, InError, OutObservable, Switcher>
where
	Switcher: Clone,
{
	fn clone(&self) -> Self {
		Self {
			switcher: self.switcher.clone(),
			_phantom_data: PhantomData,
		}
	}
}

pub struct SwitchMapSubscriber<In, InError, InnerObservable, Switcher>
where
	Switcher: Clone + Fn(In) -> InnerObservable,
{
	switcher: Switcher,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, InnerObservable, Switcher>
	SwitchMapSubscriber<In, InError, InnerObservable, Switcher>
where
	Switcher: Clone + Fn(In) -> InnerObservable,
{
	pub fn new(switcher: Switcher) -> Self {
		Self {
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

pub struct HigherOrderForwarder<S>
where
	S: LiftingForwarder,
{
	subscriber: S,
}

impl<S> HigherOrderForwarder<S>
where
	S: LiftingForwarder,
{
	pub fn new(subscriber: S) -> Self {
		Self { subscriber }
	}
}

impl<S> Forwarder for HigherOrderForwarder<S>
where
	S: LiftingForwarder,
{
	type In = ();
	type InError = ();
	type Out = ();
	type OutError = ();

	fn next_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		// self.subscriber.next_forward(next, destination);
	}

	fn error_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		next: Self::InError,
		destination: &mut Destination,
	) {
	}

	fn complete_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		destination: &mut Destination,
	) {
	}
}

impl<In, InError, OutObservable, Switcher> LiftingForwarder
	for SwitchMapSubscriber<In, InError, OutObservable, Switcher>
where
	Self: Clone,
	Switcher: Clone + Fn(In) -> OutObservable,
	OutObservable: Observable,
	InError: Into<OutObservable::Error>,
{
	type In = In;
	type InError = InError;
	type OutObservable = OutObservable;

	fn next_forward<
		LiftedDestination: Observer<In = Self::OutObservable, Error = <Self::OutObservable as Observable>::Error>,
	>(
		&mut self,
		next: Self::In,
		destination: &mut LiftedDestination,
	) {
		let next_observable = (self.switcher)(next);
		destination.next(next_observable);
	}

	fn error_forward<
		LiftedDestination: Observer<In = Self::OutObservable, Error = <Self::OutObservable as Observable>::Error>,
	>(
		&mut self,
		error: Self::InError,
		destination: &mut LiftedDestination,
	) {
		destination.error(error.into());
	}

	fn complete_forward<
		LiftedDestination: Observer<In = Self::OutObservable, Error = <Self::OutObservable as Observable>::Error>,
	>(
		&mut self,
		destination: &mut LiftedDestination,
	) {
		// TODO: Check when this should actually complete, it should wait until the inner obs is closed, is that ensured already or does it even have to be ensured here?
		destination.complete();
	}
}

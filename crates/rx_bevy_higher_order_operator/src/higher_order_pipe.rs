/*

#[inline]
pub fn higher_pipe<NextOp>(
	self,
	operator: NextOp,
) -> Pipe<Self, HigherOrderOperatorConnector<NextOp>>
where
	NextOp: HigherOrderOperator,
{
	// let next_source = operator.source_on_next(next);

	let hof = HigherOrderOperatorConnector::<NextOp> {
		higher_order_operator: operator,
	};
	let pipe = Pipe::<Self, NextOp>::new(self, hof);
} */

use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, Observable, Observer, Subscription};
use rx_bevy_observer_noop::NoopObserver;
use rx_bevy_operator::Operator;
use rx_bevy_subject::Subject;

use crate::HigherOrderOperator;

pub trait HigherOrderObserver {
	type In;
	// type Error;

	type OutObservable;

	fn source_on_next(&mut self, next: Self::In) -> Self::OutObservable;
	// fn source_on_error(&mut self, error: Self::Error) -> Self::OutObservable;
	// fn source_on_complete(&mut self);
}

pub trait HigherOrderObservable {
	type OutObservable: Observable;

	fn higher_order_subscribe<Destination: 'static + Observer<In = Self::OutObservable>>(
		&mut self,
		destination: Destination,
	) -> (); // -> <Self::OutObservable as Observable>::Subscription;
}

pub struct HigherOrderPipe<Source, Op> {
	pub(crate) source_observable: Source,
	pub(crate) operator: Op,
}

impl<Source, Op> Clone for HigherOrderPipe<Source, Op>
where
	Source: Observable + Clone,
	Op: HigherOrderOperator + Clone,
{
	fn clone(&self) -> Self {
		Self {
			operator: self.operator.clone(),
			source_observable: self.source_observable.clone(),
		}
	}
}

impl<Source, Op> HigherOrderPipe<Source, Op>
where
	Op: HigherOrderOperator,
	Source: Observable,
{
	pub fn new(source_observable: Source, operator: Op) -> Self {
		Self {
			source_observable,
			operator,
		}
	}
}

impl<Source, Op> HigherOrderPipe<Source, Op>
where
	Op: HigherOrderOperator,
	//<Op::Fw as HigherOrderOperatorSource>::InnerObservable: 'static,
	//Source: Observable<
	//		Out = <Op::Fw as HigherOrderOperatorSource>::In,
	//		Error = <Op::Fw as HigherOrderOperatorSource>::InError,
	//	>,
{
	#[inline]
	pub fn higher_order_pipe<NextOp>(self, operator: NextOp)
	//  -> HigherOrderPipe<Self, NextOp>
	where
		NextOp: HigherOrderOperator,
	{
		// HigherOrderPipe::<Self, NextOp>::new(self, operator)
	}
}
/*
impl<Source, Op> HigherOrderObservable for HigherOrderPipe<Source, Op>
where
	Op: HigherOrderOperator,
	Source: Observable<Out = Op>,
{
	type OutObservable = <Op as HigherOrderOperator>::OutObservable;

	fn higher_order_subscribe<
		Destination: 'static + HigherOrderObserver<In = Self::OutObservable>,
	>(
		&mut self,
		destination: Destination,
	) -> () {
		// -> <Self::OutObservable as Observable>::Subscription {
		// let higher_order_subscriber = self.operator.higher_order_operator_subscribe(destination);

		// self.source_observable.subscribe(higher_order_subscriber)

		// self.source_observable.subscribe(NoopObserver::new())

		()
	}
}*/
/*
impl<Source, Op> Observable for HigherOrderPipe<Source, Op>
where
	Op: HigherOrderOperator,
{
	type Out = <Op::OutObservable as Observable>::Out;
	type Error = <Op::OutObservable as Observable>::Error;
	type Subscription = <Op::OutObservable as Observable>::Subscription;

	#[inline]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::Error>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let internal_subject = Subject::<Op::OutObservable>::new();
		let operator_subscriber = self
			.operator
			.higher_order_operator_subscribe(internal_subject);

		//HigherOrderOperatorConnector
		// self.source_observable.subscribe(internal_subject)
	}
}*/

pub trait InnerObservableCreatorObserver {
	type In;
	type InError;
	type InnerObservable: Observable;

	fn source_on_next(&mut self, next: Self::In) -> Self::InnerObservable;
}
/*
pub struct InnerObservableCreatorObserver<In, Error, OutObservable> {
	_phantom_data: PhantomData<(In, Error, OutObservable)>,
}

impl<In, Error, OutObservable> Observer
	for InnerObservableCreatorObserver<In, Error, OutObservable>
{
	type In = In;
	type Error = Error;

	fn next(&mut self, next: Self::In) {}

	fn error(&mut self, error: Self::Error) {}

	fn complete(&mut self) {}
}
*/

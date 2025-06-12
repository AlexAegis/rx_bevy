use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, Observable, Observer};
use rx_bevy_observer_flat::SwitchFlattener;
use rx_bevy_operator::LiftingOperator;
use rx_bevy_pipe_flat::FlatPipe;
use rx_bevy_pipe_lift::LiftPipe;
/*
pub fn composite_switch_map<Source, Lifter>(
	source: Source,
	lifter: Lifter,
) -> FlatPipe<
	LiftPipe<Source, Lifter>,
	SwitchFlattener<
		<Lifter::Fw as LiftingForwarder>::OutObservable,
		<<Lifter::Fw as LiftingForwarder>::OutObservable as Observable>::Error,
	>,
>
where
	Source: Observable<
			Out = <Lifter::Fw as LiftingForwarder>::In,
			Error = <Lifter::Fw as LiftingForwarder>::InError,
		>,
	Lifter: LiftingOperator,
	<Lifter as LiftingOperator>::Fw: 'static,
{
	FlatPipe::new(
		LiftPipe::<Source, Lifter>::new(source, lifter),
		SwitchFlattener::new(),
	)
}*/

pub struct CompositeOperator<F, Source, Result>
where
	F: Clone + Fn(Source) -> Result,
	Source: Observable,
	Result: Observable,
{
	subscriber: F,
	_phantom_data: PhantomData<(Source, Result)>,
}

// impl<F, Source, Result> Operator for CompositeOperator<F, Source, Result>
// where
// 	F: Clone + Fn(Source) -> Result,
// 	Source: Observable,
// 	Result: Observable,
// {
// 	type Fw = CompositeForwarder<F, Source, Result>;
//
// 	fn operator_subscribe<
// 		Destination: 'static
// 			+ Observer<In = <Self::Fw as Forwarder>::Out, Error = <Self::Fw as Forwarder>::OutError>,
// 	>(
// 		&mut self,
// 		destination: Destination,
// 	) -> Subscriber<Self::Fw, Destination> {
// 		// self.subscriber()
// 		// Subscriber::new(destination, CompositeForwarder {})
// 	}
// }
//
struct CompositeForwarder<F, Source, Result>
where
	F: Clone + Fn(Source) -> Result,
	Source: Observable,
	Result: Observable,
{
	subscriber: F,
	_phantom_data: PhantomData<(Source, Result)>,
}

impl<F, Source, Result> Forwarder for CompositeForwarder<F, Source, Result>
where
	F: Clone + Fn(Source) -> Result,
	Source: Observable,
	Result: Observable,
{
	type In = Source::Out;
	type InError = Source::Error;
	type Out = Result::Out;
	type OutError = Result::Error;

	fn next_forward<Destination: Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
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

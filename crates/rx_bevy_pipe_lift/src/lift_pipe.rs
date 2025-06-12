use rx_bevy_observable::{Forwarder, Observable, Observer};
use rx_bevy_operator::LiftingOperator;

pub struct LiftPipe<Source, LiftingOp>
where
	Source: Observable<
			Out = <LiftingOp::Fw as Forwarder>::In,
			Error = <LiftingOp::Fw as Forwarder>::InError,
		>,
	LiftingOp: LiftingOperator,
{
	pub(crate) source_observable: Source,
	pub(crate) operator: LiftingOp,
}

impl<Source, LiftingOp> Clone for LiftPipe<Source, LiftingOp>
where
	Source: Observable<
			Out = <LiftingOp::Fw as Forwarder>::In,
			Error = <LiftingOp::Fw as Forwarder>::InError,
		> + Clone,
	LiftingOp: LiftingOperator + Clone,
{
	fn clone(&self) -> Self {
		Self {
			source_observable: self.source_observable.clone(),
			operator: self.operator.clone(),
		}
	}
}

impl<Source, LiftingOp> LiftPipe<Source, LiftingOp>
where
	Source: Observable<
			Out = <LiftingOp::Fw as Forwarder>::In,
			Error = <LiftingOp::Fw as Forwarder>::InError,
		>,
	LiftingOp: LiftingOperator,
{
	pub fn new(source_observable: Source, operator: LiftingOp) -> Self {
		Self {
			source_observable,
			operator,
		}
	}
}

impl<Source, LiftingOp> Observable for LiftPipe<Source, LiftingOp>
where
	Source: Observable<
			Out = <LiftingOp::Fw as Forwarder>::In,
			Error = <LiftingOp::Fw as Forwarder>::InError,
		>,
	LiftingOp: LiftingOperator<Fw: Forwarder>,
	<LiftingOp as LiftingOperator>::Fw: 'static,
{
	type Out = <LiftingOp::Fw as Forwarder>::Out;
	type Error = <LiftingOp::Fw as Forwarder>::OutError;
	type Subscription = Source::Subscription;

	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::Error>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let operator_subscriber = self
			.operator
			.lifted_operator_subscribe::<Destination>(destination);

		self.source_observable.subscribe(operator_subscriber)
	}
}

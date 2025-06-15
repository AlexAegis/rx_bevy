use crate::{
	Forwarder, IntermediateObserver, ObservableOutput, Observer, ObserverInput, Operator,
	SubscriberForwarder,
};

#[derive(Clone)]
pub struct CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator
		+ ObservableOutput<Out = <Op as ObserverInput>::In, OutError = <Op as ObserverInput>::InError>,
	Op: Operator,
{
	prev_op: PrevOp,
	op: Op,
}

impl<PrevOp, Op> CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator,
	Op: Operator,
	PrevOp::Fw: ObservableOutput<
			Out = <Op::Fw as ObserverInput>::In,
			OutError = <Op::Fw as ObserverInput>::InError,
		>,
{
	pub fn new(first_operator: PrevOp, second_operator: Op) -> Self {
		Self {
			prev_op: first_operator,
			op: second_operator,
		}
	}

	pub fn pipe<NextOp>(self, next_operator: NextOp) -> CompositeOperator<Self, NextOp>
	where
		NextOp: Operator,
		NextOp::Fw: ObserverInput<
				In = <Op as ObservableOutput>::Out,
				InError = <Op as ObservableOutput>::OutError,
			>,
	{
		CompositeOperator {
			prev_op: self,
			op: next_operator,
		}
	}
}

impl<PrevOp, Op> Operator for CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator,
	Op: Operator,
	PrevOp::Fw: ObservableOutput<
			Out = <Op::Fw as ObserverInput>::In,
			OutError = <Op::Fw as ObserverInput>::InError,
		>,
{
	type Fw = CompositeForwarder<PrevOp::Fw, Op::Fw>;

	fn create_instance(&self) -> Self::Fw {
		CompositeForwarder {
			prev_op: self.prev_op.create_instance(),
			op: self.op.create_instance(),
		}
	}
}

#[derive(Clone)]
pub struct CompositeForwarder<PrevFw, Fw>
where
	PrevFw: Forwarder
		+ ObservableOutput<Out = <Fw as ObserverInput>::In, OutError = <Fw as ObserverInput>::InError>,
	Fw: Forwarder,
{
	pub prev_op: PrevFw,
	pub op: Fw,
}

impl<PrevFw, Fw> ObserverInput for CompositeForwarder<PrevFw, Fw>
where
	PrevFw: Forwarder
		+ ObservableOutput<Out = <Fw as ObserverInput>::In, OutError = <Fw as ObserverInput>::InError>,
	Fw: Forwarder,
{
	type In = <PrevFw as ObserverInput>::In;
	type InError = <PrevFw as ObserverInput>::InError;
}

impl<PrevFw, Fw> ObservableOutput for CompositeForwarder<PrevFw, Fw>
where
	PrevFw: Forwarder
		+ ObservableOutput<Out = <Fw as ObserverInput>::In, OutError = <Fw as ObserverInput>::InError>,
	Fw: Forwarder,
{
	type Out = <Fw as ObservableOutput>::Out;
	type OutError = <Fw as ObservableOutput>::OutError;
}

impl<PrevFw, Fw> Forwarder for CompositeForwarder<PrevFw, Fw>
where
	PrevFw: Forwarder
		+ ObservableOutput<Out = <Fw as ObserverInput>::In, OutError = <Fw as ObserverInput>::InError>,
	Fw: Forwarder,
{
	fn next_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		let mut intermediate_observer = IntermediateObserver::new(&mut self.op, destination);
		self.prev_op.next_forward(next, &mut intermediate_observer);
	}

	fn error_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		let mut intermediate_observer = IntermediateObserver::new(&mut self.op, destination);
		self.prev_op
			.error_forward(error, &mut intermediate_observer);
	}

	fn complete_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: &mut Destination,
	) {
		let mut intermediate_observer = IntermediateObserver::new(&mut self.op, destination);
		self.prev_op.complete_forward(&mut intermediate_observer);
	}
}

pub trait CompositeOperatorExtension: Operator + Sized {
	fn pipe<NextOp>(self, next_operator: NextOp) -> CompositeOperator<Self, NextOp>
	where
		NextOp: Operator,
		NextOp::Fw: ObserverInput<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	{
		CompositeOperator {
			prev_op: self,
			op: next_operator,
		}
	}
}

impl<T> CompositeOperatorExtension for T where T: Operator {}

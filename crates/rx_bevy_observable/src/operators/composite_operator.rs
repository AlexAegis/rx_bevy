use std::marker::PhantomData;

use crate::{
	IntermediateObserver, ObservableOutput, Observer, ObserverInput, Operator, SubscriberForwarder,
};

#[derive(Clone)]
pub struct CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator,
	PrevOp: ObservableOutput<Out = <Op as ObserverInput>::In, OutError = <Op as ObserverInput>::InError>,
	Op: Operator,
{
	prev_op: PrevOp,
	op: Op,
}

impl<PrevOp, Op> CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator,
	PrevOp: ObservableOutput<Out = <Op as ObserverInput>::In, OutError = <Op as ObserverInput>::InError>,
	Op: Operator,
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
		NextOp: ObserverInput<
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
	PrevOp: ObservableOutput<Out = <Op as ObserverInput>::In, OutError = <Op as ObserverInput>::InError>,
	Op: Operator,
{
	type Subscriber<D>
		= CompositeForwarder<PrevOp::Subscriber<D>, Op::Subscriber<D>, D>
	where
		D: Observer<In = Self::Out, InError = Self::OutError>;

	fn create_instance<D>(&self) -> Self::Subscriber<D>
	where
		D: Observer<In = Self::Out, InError = Self::OutError>,
	{
		CompositeForwarder {
			prev_op: self.prev_op.create_instance(),
			op: self.op.create_instance(),
			_phantom_data: PhantomData,
		}
	}
}

impl<PrevOp, Op> ObserverInput for CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator,
	PrevOp: ObservableOutput<Out = <Op as ObserverInput>::In, OutError = <Op as ObserverInput>::InError>,
	Op: Operator,
{
	type In = PrevOp::In;
	type InError = PrevOp::InError;
}

impl<PrevOp, Op> ObservableOutput for CompositeOperator<PrevOp, Op>
where
	PrevOp: Operator,
	PrevOp: ObservableOutput<Out = <Op as ObserverInput>::In, OutError = <Op as ObserverInput>::InError>,
	Op: Operator,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

#[derive(Clone)]
pub struct CompositeForwarder<PrevFw, Fw, D>
where
	PrevFw: SubscriberForwarder<Destination = Fw::Destination>
		+ ObservableOutput<Out = Fw::In, OutError = Fw::InError>,
	Fw: SubscriberForwarder<Destination = D>,
	D: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	pub prev_op: PrevFw,
	pub op: Fw,
	_phantom_data: PhantomData<D>,
}

impl<PrevFw, Fw, D> ObserverInput for CompositeForwarder<PrevFw, Fw, D>
where
	PrevFw: SubscriberForwarder<Destination = Fw::Destination>
		+ ObservableOutput<Out = Fw::In, OutError = Fw::InError>,
	Fw: SubscriberForwarder<Destination = D>,
	D: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	type In = <PrevFw as ObserverInput>::In;
	type InError = <PrevFw as ObserverInput>::InError;
}

impl<PrevFw, Fw, D> ObservableOutput for CompositeForwarder<PrevFw, Fw, D>
where
	PrevFw: SubscriberForwarder<Destination = Fw::Destination>
		+ ObservableOutput<Out = Fw::In, OutError = Fw::InError>,
	Fw: SubscriberForwarder<Destination = D>,
	D: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	type Out = <Fw as ObservableOutput>::Out;
	type OutError = <Fw as ObservableOutput>::OutError;
}

impl<PrevFw, Fw, D> SubscriberForwarder for CompositeForwarder<PrevFw, Fw, D>
where
	PrevFw: for<'a> SubscriberForwarder + ObservableOutput<Out = Fw::In, OutError = Fw::InError>,
	Fw: SubscriberForwarder<Destination = D>,
	D: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	type Destination = D;

	fn next_forward(&mut self, next: Self::In, destination: &mut Self::Destination) {
		let mut intermediate_observer = IntermediateObserver::new(&mut self.op, destination);
		self.prev_op.next_forward(next, &mut intermediate_observer);
	}

	fn error_forward(&mut self, error: Self::InError, destination: &mut Self::Destination) {
		let mut intermediate_observer = IntermediateObserver::new(&mut self.op, destination);
		self.prev_op
			.error_forward(error, &mut intermediate_observer);
	}

	fn complete_forward(&mut self, destination: &mut Self::Destination) {
		let mut intermediate_observer = IntermediateObserver::new(&mut self.op, destination);
		self.prev_op.complete_forward(&mut intermediate_observer);
	}
}

pub trait CompositeOperatorExtension: Operator + Sized {
	fn pipe<NextOp>(self, next_operator: NextOp) -> CompositeOperator<Self, NextOp>
	where
		NextOp: Operator,
		NextOp: ObserverInput<
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

use crate::{ComposableOperator, Observable, Operator, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use super::Pipe;

#[derive(RxOperator, Clone, Debug)]
#[_rx_core_traits_crate(crate)]
#[rx_in(Op::In)]
#[rx_in_error(Op::InError)]
#[rx_out(Op::Out)]
#[rx_out_error(Op::OutError)]
pub struct ComposeOperator<Op>
where
	Op: 'static + ComposableOperator,
{
	operator: Op,
}

impl<Op> ComposeOperator<Op>
where
	Op: 'static + ComposableOperator,
{
	#[inline]
	pub fn new(operator: Op) -> Self {
		Self { operator }
	}
}

impl<Op> From<Op> for ComposeOperator<Op>
where
	Op: 'static + ComposableOperator,
{
	#[inline]
	fn from(operator: Op) -> Self {
		ComposeOperator::new(operator)
	}
}

impl<Op> ComposableOperator for ComposeOperator<Op>
where
	Op: 'static + ComposableOperator,
{
	type Subscriber<Destination>
		= Op::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		self.operator.operator_subscribe(destination)
	}
}

impl<'o, Op> Operator<'o> for Op
where
	Op: 'static + ComposableOperator,
{
	type OutObservable<InObservable>
		= Pipe<InObservable, Op>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync;

	#[inline]
	fn operate<InObservable>(self, source: InObservable) -> Self::OutObservable<InObservable>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync,
	{
		Pipe::new(source, self)
	}
}

use std::marker::PhantomData;

use rx_bevy_observable::{Observable, Observer, ObserverConnector};
use rx_bevy_operator::Operator;

pub struct Pipe<Source, Op, PipeInError, PipeOutError, PipeIn, PipeOut> {
	pub(crate) source_observable: Source,
	pub(crate) operator: Op,
	_phantom_data_in: PhantomData<PipeIn>,
	_phantom_data_out: PhantomData<PipeOut>,
	_phantom_data_in_error: PhantomData<PipeInError>,
	_phantom_data_out_error: PhantomData<PipeOutError>,
}

impl<Source, Op, PipeInError, PipeOutError, PipeIn, PipeOut> Clone
	for Pipe<Source, Op, PipeInError, PipeOutError, PipeIn, PipeOut>
where
	Source: Clone,
	Op: Clone,
{
	fn clone(&self) -> Self {
		Self {
			operator: self.operator.clone(),
			source_observable: self.source_observable.clone(),
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
			_phantom_data_in_error: PhantomData,
			_phantom_data_out_error: PhantomData,
		}
	}
}

impl<Source, Op, PipeInError, PipeOutError, PipeIn, PipeOut>
	Pipe<Source, Op, PipeInError, PipeOutError, PipeIn, PipeOut>
{
	pub fn new(source_observable: Source, operator: Op) -> Self {
		Self {
			source_observable,
			operator,
			_phantom_data_in: PhantomData,
			_phantom_data_out: PhantomData,
			_phantom_data_in_error: PhantomData,
			_phantom_data_out_error: PhantomData,
		}
	}
}

impl<Source, Op, PipeInError, PipeOutError, PipeIn, PipeOut>
	Pipe<Source, Op, PipeInError, PipeOutError, PipeIn, PipeOut>
{
	pub fn pipe<NextOp>(
		self,
		operator: NextOp,
	) -> Pipe<Self, NextOp, PipeInError, NextOp::OutError, PipeIn, NextOp::Out>
	where
		NextOp: Operator,
	{
		Pipe::<Self, NextOp, PipeInError, NextOp::OutError, PipeIn, NextOp::Out>::new(
			self, operator,
		)
	}
}

impl<Source, Op, PipeInError, PipeOutError, PipeIn, PipeOut> Observable
	for Pipe<Source, Op, PipeInError, PipeOutError, PipeIn, PipeOut>
where
	Op: Operator<Out = PipeOut, OutError = PipeOutError>,
	Source: Observable<Out = Op::In, Error = PipeInError>,
	<Op as Operator>::InternalSubscriber:
		ObserverConnector<In = Op::In, InError = PipeInError> + 'static,
{
	type Out = PipeOut;
	type Error = PipeOutError;
	type Subscription = <Source as Observable>::Subscription;

	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::Error>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let operator_subscriber = self.operator.operator_subscribe(destination);
		self.source_observable.subscribe(operator_subscriber)
	}
}

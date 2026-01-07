use crate::{ComposableOperator, Observable, Subscriber, UpgradeableObserver};
use rx_core_macro_observable_derive::RxObservable;

#[derive(RxObservable, Clone, Debug)]
#[_rx_core_common_crate(crate)]
#[rx_out(Op::Out)]
#[rx_out_error(Op::OutError)]
pub struct Pipe<Source, Op>
where
	Source: Observable,
	Op: 'static + ComposableOperator<In = Source::Out, InError = Source::OutError>,
{
	source_observable: Source,
	operator: Op,
}

impl<Source, Op> Pipe<Source, Op>
where
	Source: Observable,
	Op: 'static + ComposableOperator<In = Source::Out, InError = Source::OutError>,
{
	pub fn new(source_observable: Source, operator: Op) -> Self {
		Self {
			source_observable,
			operator,
		}
	}
}

impl<Source, Op> Observable for Pipe<Source, Op>
where
	Source: Observable,
	Op: 'static + ComposableOperator<In = Source::Out, InError = Source::OutError>,
{
	type Subscription<Destination>
		= Source::Subscription<
		<Op as ComposableOperator>::Subscriber<<Destination as UpgradeableObserver>::Upgraded>,
	>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	#[inline]
	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let destination = observer.upgrade();
		let operator_subscriber = self.operator.operator_subscribe(destination);
		self.source_observable.subscribe(operator_subscriber)
	}
}

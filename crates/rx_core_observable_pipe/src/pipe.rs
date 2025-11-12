use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Observable, ObservableOutput, Operator, Subscriber, SubscriptionContext, UpgradeableObserver,
	WithSubscriptionContext,
};

#[derive(RxObservable, Clone, Debug)]
#[rx_out(Op::Out)]
#[rx_out_error(Op::OutError)]
#[rx_context(Source::Context)]
pub struct Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
{
	pub(crate) source_observable: Source,
	pub(crate) operator: Op,
}

impl<Source, Op> Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
{
	pub fn new(source_observable: Source, operator: Op) -> Self {
		Self {
			source_observable,
			operator,
		}
	}
}

impl<Source, Op> Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
{
	#[inline]
	pub fn pipe<NextOp>(self, operator: NextOp) -> Pipe<Self, NextOp>
	where
		NextOp: 'static
			+ Operator<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
				Context = <Self as WithSubscriptionContext>::Context,
			>,
	{
		Pipe::<Self, NextOp>::new(self, operator)
	}
}

impl<Source, Op> Observable for Pipe<Source, Op>
where
	Source: 'static + Observable,
	Op: 'static + Operator<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
{
	type Subscription<Destination>
		= Source::Subscription<
		<Op as Operator>::Subscriber<<Destination as UpgradeableObserver>::Upgraded>,
	>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	#[inline]
	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let destination = observer.upgrade();
		let operator_subscriber = self.operator.operator_subscribe(destination, context);
		self.source_observable
			.subscribe(operator_subscriber, context)
	}
}

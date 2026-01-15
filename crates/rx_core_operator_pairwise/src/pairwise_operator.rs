use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Never, PhantomInvariant, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::PairwiseSubscriber;

#[derive_where(Debug, Clone, Default)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out([In; 2])]
#[rx_out_error(InError)]
pub struct PairwiseOperator<In, InError = Never>
where
	In: Signal + Clone,
	InError: Signal,
{
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> ComposableOperator for PairwiseOperator<In, InError>
where
	In: Signal + Clone,
	InError: Signal,
{
	type Subscriber<Destination>
		= PairwiseSubscriber<In, Destination>
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
		PairwiseSubscriber::new(destination)
	}
}

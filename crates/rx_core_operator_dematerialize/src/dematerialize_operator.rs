use derive_where::derive_where;
use rx_core_common::{
	ComposableOperator, Never, ObserverNotification, PhantomInvariant, Signal, Subscriber,
};
use rx_core_macro_operator_derive::RxOperator;

use crate::DematerializeSubscriber;

/// The [DematerializeOperator] is used to pack all signals, nexts, errors,
/// completes into purely `next` emissions as [ObserverNotification]s.
#[derive(RxOperator)]
#[derive_where(Debug, Clone, Default)]
#[rx_in(ObserverNotification<In, InError>)]
#[rx_in_error(Never)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct DematerializeOperator<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> ComposableOperator for DematerializeOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= DematerializeSubscriber<Destination>
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
		DematerializeSubscriber::new(destination)
	}
}

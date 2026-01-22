use derive_where::derive_where;
use rx_core_common::{ComposableOperator, PhantomInvariant, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::CountSubscriber;

/// # [CountOperator]
///
/// The `count` operator counts upstream emissions and emits the total once
/// upstream completes.
#[derive_where(Debug, Clone, Default)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(usize)]
#[rx_out_error(InError)]
pub struct CountOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> ComposableOperator for CountOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= CountSubscriber<In, InError, Destination>
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
		CountSubscriber::new(destination)
	}
}

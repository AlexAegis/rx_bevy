use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Never, ObserverNotification, Signal, Subscriber};

use crate::MaterializeSubscriber;

/// The [MaterializeOperator] is used to pack all signals, nexts, errors,
/// completes into purely `next` emissions as [ObserverNotification]s.
#[derive(RxOperator)]
#[derive_where(Debug, Clone, Default)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(ObserverNotification<In, InError>)]
#[rx_out_error(Never)]
pub struct MaterializeOperator<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> ComposableOperator for MaterializeOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= MaterializeSubscriber<In, InError, Destination>
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
		MaterializeSubscriber::new(destination)
	}
}

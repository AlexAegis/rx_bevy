use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Never, Signal, Subscriber};

use crate::ErrorBoundarySubscriber;

/// # [ErrorBoundaryOperator]
///
/// The [ErrorBoundaryOperator] does nothing but ensure that the error type is
/// [Never], giving you a compile error if the upstream error type has
/// accidentally changed to something else, when you want to ensure the
/// downstream error type stays [Never].
#[derive_where(Debug, Default, Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(Never)]
#[rx_out(In)]
#[rx_out_error(Never)]
pub struct ErrorBoundaryOperator<In>
where
	In: Signal,
{
	_phantom_data: PhantomData<In>,
}

impl<In> ComposableOperator for ErrorBoundaryOperator<In>
where
	In: Signal,
{
	type Subscriber<Destination>
		= ErrorBoundarySubscriber<Destination>
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
		ErrorBoundarySubscriber::new(destination)
	}
}

use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Never, Operator, SignalBound, Subscriber, SubscriptionContext};

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
#[rx_context(Context)]
pub struct ErrorBoundaryOperator<In, Context>
where
	In: SignalBound,
	Context: SubscriptionContext,
{
	_phantom_data: PhantomData<(In, fn(Context))>,
}

impl<In, Context> Operator for ErrorBoundaryOperator<In, Context>
where
	In: SignalBound,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= ErrorBoundarySubscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		ErrorBoundarySubscriber::new(destination)
	}
}

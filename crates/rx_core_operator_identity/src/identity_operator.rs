use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Never, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::IdentitySubscriber;

/// # [IdentityOperator]
///
/// The [IdentityOperator] does nothing. Its only purpose is to let you
/// easily define input types for a [CompositeOperator].
#[derive(RxOperator)]
#[derive_where(Default, Clone, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct IdentityOperator<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> ComposableOperator for IdentityOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= IdentitySubscriber<Destination>
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
		IdentitySubscriber::new(destination)
	}
}

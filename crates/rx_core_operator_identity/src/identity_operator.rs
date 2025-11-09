use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Never, Operator, SignalBound, Subscriber, SubscriptionContext};

use crate::IdentitySubscriber;

/// # [IdentityOperator]
///
/// The [IdentityOperator] does nothing. It's only purpose is to let you
/// easily define input types for a [CompositeOperator]
#[derive(RxOperator)]
#[derive_where(Default, Clone, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct IdentityOperator<In, InError = Never, Context = ()>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> Operator for IdentityOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= IdentitySubscriber<Destination>
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
		IdentitySubscriber::new(destination)
	}
}

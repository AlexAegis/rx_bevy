use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Signal, Subscriber, SubscriptionContext};

use crate::EnumerateSubscriber;

/// The [EnumerateOperator] counts emissions, and downstream receives this
/// counter in a tuple with the emitted value as (T, usize)
#[derive_where(Default, Clone, Debug)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out((In, usize))]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct EnumerateOperator<In, InError, Context = ()>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> Operator for EnumerateOperator<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= EnumerateSubscriber<In, Destination>
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
		EnumerateSubscriber::new(destination)
	}
}

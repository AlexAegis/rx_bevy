use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Never, Operator, SignalBound, Subscriber, SubscriptionContext};

use crate::SkipSubscriber;

/// The [SkipOperator] is used to skip the first `n` emissions of an observable,
/// after which it does nothing.
#[derive(RxOperator)]
#[derive_where(Debug, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct SkipOperator<In, InError = Never, Context = ()>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	count: usize,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> SkipOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	pub fn new(count: usize) -> Self {
		Self {
			count,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> Operator for SkipOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= SkipSubscriber<Destination>
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
		SkipSubscriber::new(destination, self.count)
	}
}

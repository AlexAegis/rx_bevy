use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Never, Operator, Signal, Subscriber, SubscriptionContext};

use crate::{DelaySubscriber, operator::DelayOperatorOptions};

#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct DelayOperator<In, InError = Never, Context = ()>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	options: DelayOperatorOptions,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context> DelayOperator<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	pub fn new(options: DelayOperatorOptions) -> Self {
		Self {
			options,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context> Operator for DelayOperator<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= DelaySubscriber<Destination>
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
		DelaySubscriber::new(destination, self.options)
	}
}

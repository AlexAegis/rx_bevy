use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Scheduler, Signal, Subscriber, SubscriptionContext};

use crate::{DelaySubscriber, operator::DelayOperatorOptions};

#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct DelayOperator<In, InError, Context, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
	Context: SubscriptionContext,
{
	options: DelayOperatorOptions<S>,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Context, S> DelayOperator<In, InError, Context, S>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
	S: Scheduler,
{
	pub fn new(options: DelayOperatorOptions<S>) -> Self {
		Self {
			options,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Context, S> Operator for DelayOperator<In, InError, Context, S>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
	S: 'static + Scheduler,
{
	type Subscriber<Destination>
		= DelaySubscriber<Destination, S>
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
		DelaySubscriber::new(destination, self.options.clone())
	}
}

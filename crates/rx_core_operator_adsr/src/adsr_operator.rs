use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, SignalBound, Subscriber, SubscriptionContext};

use crate::{AdsrSignal, AdsrSubscriber, operator::AdsrOperatorOptions};

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(RxOperator)]
#[rx_in(bool)]
#[rx_in_error(InError)]
#[rx_out(AdsrSignal)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct AdsrOperator<InError, Context>
where
	InError: SignalBound,
	Context: SubscriptionContext,
{
	options: AdsrOperatorOptions,
	_phantom_data: PhantomData<(InError, fn(Context))>,
}

impl<InError, Context> AdsrOperator<InError, Context>
where
	InError: SignalBound,
	Context: SubscriptionContext,
{
	pub fn new(options: AdsrOperatorOptions) -> Self {
		Self {
			options,
			_phantom_data: PhantomData,
		}
	}
}

impl<InError, Context> Operator for AdsrOperator<InError, Context>
where
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= AdsrSubscriber<InError, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

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
		AdsrSubscriber::new(destination, self.options.clone())
	}
}

use core::marker::PhantomData;

use rx_core_traits::{
	ObservableOutput, ObserverInput, Operator, SignalBound, Subscriber, SubscriptionContext,
};

use crate::{AdsrSignal, AdsrSubscriber, operator::AdsrOperatorOptions};

// TODO: Currently this is a regular operator, not an operatorComponent, which would make it hard to control it from bevy
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct AdsrOperator<InError, Context>
where
	InError: SignalBound,
	Context: SubscriptionContext + 'static,
{
	options: AdsrOperatorOptions,
	_phantom_data: PhantomData<(InError, fn(Context))>,
}

impl<InError, Context> AdsrOperator<InError, Context>
where
	InError: SignalBound,
	Context: SubscriptionContext + 'static,
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
	Context: SubscriptionContext + 'static,
{
	type Context = Context;

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

impl<InError, Context> ObserverInput for AdsrOperator<InError, Context>
where
	InError: SignalBound,
	Context: SubscriptionContext + 'static,
{
	type In = bool;
	type InError = InError;
}

impl<InError, Context> ObservableOutput for AdsrOperator<InError, Context>
where
	InError: SignalBound,
	Context: SubscriptionContext + 'static,
{
	type Out = AdsrSignal;
	type OutError = InError;
}

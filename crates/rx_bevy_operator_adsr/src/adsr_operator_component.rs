use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, ObservableOutput, ObserverInput, Operator, Subscriber, SubscriptionCollection,
};

use crate::{AdsrOperatorOptions, AdsrSignal, AdsrSubscriber};

// TODO: Currently this is a regular operator, not an operatorComponent, which would make it hard to control it from bevy
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct AdsrOperator<InError, Context>
where
	InError: 'static,
	Context: DropContext + 'static,
{
	options: AdsrOperatorOptions,
	_phantom_data: PhantomData<(*mut InError, *mut Context)>,
}

impl<InError, Context> AdsrOperator<InError, Context>
where
	InError: 'static,
	Context: DropContext + 'static,
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
	InError: 'static,
	Context: DropContext + 'static,
{
	type Context = Context;

	type Subscriber<Destination>
		= AdsrSubscriber<InError, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection,
	{
		AdsrSubscriber::new(destination, self.options.clone())
	}
}

impl<InError, Context> ObserverInput for AdsrOperator<InError, Context>
where
	InError: 'static,
	Context: DropContext + 'static,
{
	type In = bool;
	type InError = InError;
}

impl<InError, Context> ObservableOutput for AdsrOperator<InError, Context>
where
	InError: 'static,
	Context: DropContext + 'static,
{
	type Out = AdsrSignal;
	type OutError = InError;
}

use std::marker::PhantomData;

use rx_bevy_common_bounds::SignalBound;
use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, SignalContext, Subscriber};

use crate::{AdsrOperatorOptions, AdsrSignal, AdsrSubscriber};

// TODO: Currently this is a regular operator, not an operatorComponent, which would make it hard to control it from bevy
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct AdsrOperator<InError> {
	options: AdsrOperatorOptions,
	_phantom_data: PhantomData<InError>,
}

impl<InError> AdsrOperator<InError> {
	pub fn new(options: AdsrOperatorOptions) -> Self {
		Self {
			options,
			_phantom_data: PhantomData,
		}
	}
}

impl<InError> Operator for AdsrOperator<InError>
where
	InError: SignalBound,
{
	type Subscriber<Destination>
		= AdsrSubscriber<InError, Destination>
	where
		Destination: Subscriber<In = Self::Out, InError = Self::OutError>;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Self::Subscriber<Destination> as SignalContext>::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: Subscriber<In = Self::Out, InError = Self::OutError>,
	{
		AdsrSubscriber::new(destination, self.options.clone())
	}
}

impl<InError> ObserverInput for AdsrOperator<InError>
where
	InError: 'static,
{
	type In = bool;
	type InError = InError;
}

impl<InError> ObservableOutput for AdsrOperator<InError>
where
	InError: 'static,
{
	type Out = AdsrSignal;
	type OutError = InError;
}

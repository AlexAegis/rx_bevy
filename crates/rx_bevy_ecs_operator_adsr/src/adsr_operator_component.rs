use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, Subscriber};
use rx_bevy_plugin::SignalBound;

use crate::{AdsrOperatorOptions, AdsrSignal, AdsrSubscriber};

// TODO: Currently this is a regular operator, not an operatorComponent, which would make it hard to control it from bevy
#[derive(Debug, Clone)]
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
	type Subscriber<D: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		AdsrSubscriber<InError, D>;

	fn operator_subscribe<
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
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

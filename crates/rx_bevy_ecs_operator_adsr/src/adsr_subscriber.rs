use std::marker::PhantomData;

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike, Tick,
};

use crate::{AdsrEnvelopePhase, AdsrEnvelopeState, AdsrOperatorOptions, AdsrSignal};

// TODO: It'd be nice to control the envelope live, I guess that could be done by querying the subscriber itself, but it would be nicer to control the operator itself, in case there are many observers
#[derive(Debug)]
pub struct AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
{
	destination: Destination,
	is_getting_activated: bool,
	state: AdsrEnvelopeState,
	pub options: AdsrOperatorOptions,
	_phantom_data: PhantomData<InError>,
}

impl<InError, Destination> AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
{
	pub fn new(destination: Destination, options: AdsrOperatorOptions) -> Self {
		Self {
			destination,
			options,
			is_getting_activated: false,
			state: AdsrEnvelopeState::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<InError, Destination> Observer for AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
	InError: 'static,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.is_getting_activated = next;
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}

	#[inline]
	fn tick(&mut self, tick: Tick) {
		let next =
			self.state
				.calculate_output(self.options.envelope, self.is_getting_activated, tick);

		if !matches!(next.adsr_envelope_phase, AdsrEnvelopePhase::None) {
			self.destination.next(next);
		}
	}
}

impl<InError, Destination> SubscriptionLike for AdsrSubscriber<InError, Destination>
where
	Destination: Subscriber<In = AdsrSignal, InError = InError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: Box<dyn SubscriptionLike>) {
		self.destination.add(subscription);
	}
}

impl<InError, Destination> ObserverInput for AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
	InError: 'static,
{
	type In = bool;
	type InError = InError;
}

impl<InError, Destination> ObservableOutput for AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
	InError: 'static,
{
	type Out = AdsrSignal;
	type OutError = InError;
}

impl<InError, Destination> Operation for AdsrSubscriber<InError, Destination>
where
	Destination: Observer<In = AdsrSignal, InError = InError>,
{
	type Destination = Destination;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}

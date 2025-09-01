use rx_bevy_core::{Observable, ObservableOutput, Subscriber, Subscription};

/// Dyn compatible Observable for internal cases where the destination is known
pub trait FixedSubscriberObservable<Destination>: ObservableOutput
where
	Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>,
{
	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe(&mut self, destination: Destination) -> Subscription;
}

impl<O, Destination> FixedSubscriberObservable<Destination> for O
where
	O: Observable,
	Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>,
{
	fn subscribe(&mut self, destination: Destination) -> Subscription {
		Observable::subscribe(self, destination)
	}
}

pub trait IntoFixedSubscriberObservable: Observable {
	type FixedSubscriberObservable<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>>: FixedSubscriberObservable<Destination, Out = Self::Out, OutError = Self::OutError>;

	fn into_fixed_subscriber_observable<
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>,
	>(
		self,
		destination: Destination,
	) -> Self::FixedSubscriberObservable<Destination>;
}

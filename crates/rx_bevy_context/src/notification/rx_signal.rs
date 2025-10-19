/// Shorthand for [SubscriberNotificationEvent]
// TODO: use this or don't pub type RxSignal<In, InError> = InternalSubscriberNotificationEvent<In, InError>;

#[derive(Event, Clone, Debug)]
pub enum RxSignal<In, InError = ()>
where
	In: SignalBound,
	InError: SignalBound,
{
	Next(In),
	Error(InError),
	Complete,
}

impl<In, InError> From<RxSignal<In, InError>> for SubscriberNotificationEvent<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(value: RxSignal<In, InError>) -> Self {
		match value {
			RxSignal::Next(next) => SubscriberNotificationEvent::Next(next),
			RxSignal::Error(error) => SubscriberNotificationEvent::Error(error),
			RxSignal::Complete => SubscriberNotificationEvent::Complete,
		}
	}
}

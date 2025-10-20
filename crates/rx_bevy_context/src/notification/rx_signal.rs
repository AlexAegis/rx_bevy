use crate::ConsumableSubscriberNotificationEvent;

pub type RxSignal<In, InError = ()> = ConsumableSubscriberNotificationEvent<In, InError>;

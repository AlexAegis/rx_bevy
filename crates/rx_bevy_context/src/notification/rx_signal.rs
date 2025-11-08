use rx_core_traits::Never;

use crate::ConsumableSubscriberNotificationEvent;

pub type RxSignal<In, InError = Never> = ConsumableSubscriberNotificationEvent<In, InError>;

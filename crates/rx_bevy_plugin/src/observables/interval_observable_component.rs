use bevy::prelude::*;
use rx_bevy::ObservableOutput;

use std::time::Duration;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IntervalObserverComponent {
	interval: Duration,
}

impl ObservableOutput for IntervalObserverComponent {
	type Out = i32;
	type OutError = ();
}

use bevy_input::keyboard::KeyCode;
use rx_bevy_common::RxBevyScheduler;
use rx_core_common::{Never, Observable, SchedulerHandle, Subscriber, UpgradeableObserver};
use rx_core_macro_observable_derive::RxObservable;

use crate::{KeyboardObservableOptions, KeyboardSubscription};

/// A simplistic observable to demonstrate accessing world state from within a
/// subscription
#[derive(RxObservable)]
#[rx_out(KeyCode)]
#[rx_out_error(Never)]
pub struct KeyboardObservable {
	options: KeyboardObservableOptions,
	scheduler: SchedulerHandle<RxBevyScheduler>,
}

impl KeyboardObservable {
	pub fn new(
		options: KeyboardObservableOptions,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self {
		Self { options, scheduler }
	}
}

impl Observable for KeyboardObservable {
	type Subscription<Destination>
		= KeyboardSubscription<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		KeyboardSubscription::new(
			destination.upgrade(),
			self.options.clone(),
			self.scheduler.clone(),
		)
	}
}

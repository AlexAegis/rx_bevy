use bevy_input::keyboard::KeyCode;
use rx_bevy_context::RxBevyContext;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Never, Observable, Subscriber, SubscriptionContext, UpgradeableObserver};

use crate::{KeyboardObservableOptions, KeyboardSubscription};

/// A simplistic observable to demonstrate accessing world state from within a
/// subscription
#[derive(RxObservable, Default)]
#[rx_out(KeyCode)]
#[rx_out_error(Never)]
#[rx_context(RxBevyContext)]
pub struct KeyboardObservable {
	options: KeyboardObservableOptions,
}

impl KeyboardObservable {
	pub fn new(options: KeyboardObservableOptions) -> Self {
		Self { options }
	}
}

impl Observable for KeyboardObservable {
	type Subscription<Destination>
		= KeyboardSubscription<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		KeyboardSubscription::new(destination.upgrade(), self.options.clone())
	}
}

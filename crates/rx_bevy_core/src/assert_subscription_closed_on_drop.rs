use short_type_name::short_type_name;

use crate::SubscriptionLike;

pub trait AssertSubscriptionClosedOnDrop: SubscriptionLike {
	fn assert_closed_when_dropped(&self) {
		if !self.is_closed() {
			let message = format!(
				"{} was dropped without unsubscribing first!",
				short_type_name::<Self>()
			);
			#[cfg(all(
				feature = "bevy",
				not(feature = "dev_panic_on_dropped_active_subscriptions")
			))]
			bevy_log::warn!("{}", message);
			#[cfg(all(
				not(feature = "bevy"),
				not(feature = "dev_panic_on_dropped_active_subscriptions")
			))]
			println!("{}", message);
			#[cfg(feature = "dev_panic_on_dropped_active_subscriptions")]
			panic!("{}", message);
		}
	}
}

impl<T> AssertSubscriptionClosedOnDrop for T where T: SubscriptionLike {}

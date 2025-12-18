use quote::quote;
use rx_core_macro_common::{
	impl_delegate_observer_to_destination, impl_delegate_subscription_like_to_destination,
	impl_delegate_teardown_collection, impl_observer_input, impl_observer_upgrades_to,
	impl_primary_category, impl_skip_unsubscribe_on_drop_impl,
	impl_subscriber_does_not_upgrade_to_self,
};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_subscriber() -> Type {
	parse_quote! {
		PrimaryCategorySubscriber
	}
}

/// # RxSubscriber
///
/// Helper macro to implement a few traits required for a subscriber.
///
/// ## Traits you still have to implement to get a subscriber
///
/// - `Observer` (unless using `#[rx_delegate_observer_to_destination]`)
/// - `SubscriptionLike` (unless using
///   `#[rx_delegate_subscription_like_to_destination]`)
/// - `TeardownCollection` (unless using `#[rx_delegate_teardown_collection]`)
///
/// ## Traits Implemented
///
/// - `WithPrimaryCategory`: Sets the associated type to
///   `PrimaryCategorySubscription`
/// - `ObserverInput`: Sets the associated type `In` to the value of the
///   `#[rx_in(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
///   sets the associated `InError` type to the value of the
///   `#[rx_in_error(...)]` attribute, or to `Never` if missing.
/// - `UpgradeableObserver`: By default. It implements `UpgradeableObserver` by
///   just returning itself as is. This implementation can
///   be opted out with the `#[rx_does_not_upgrade_to_self]` attribute to
///   provide a manual implementation. Other preset implementations can be
///   used with the `#[rx_upgrades_to(...)]` attribute.
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_in(...)]` (optional, default: `Never`): Defines the input type of
///   the subscriber
/// - `#[rx_in_error(...)]` (optional, default: `Never`): Defines the input
///   error type of the subscriber
/// - `#[rx_does_not_upgrade_to_self]` (optional): Opts out the default
///   `UpgradeableObserver` implementation which just returns the subscriber
///   to be directly used as a destination for an `Observable` to
///   let upstream call unsubscribe on the subscriber.
/// - `#[rx_upgrades_to(...)]` (optional, accepts: `self`,
///   `observer_subscriber`): Defines a preset implementation for
///   `UpgradeableObserver`
///   - `self`: Upgraded version is itself, causing it to be unsubscribed
///     when upstream is unsubscribed when used as an observables destination.
///   - `observer_subscriber`: Upgraded version is itself wrapped in
///     `ObserverSubscriber`, causing it to **not** be unsubscribed when
///     upstream is unsubscribed when used as an observables destination.
/// - `#[rx_delegate_teardown_collection]` (optional): Opts into
///   the trivial implementation of `TeardownCollection` where the traits
///   methods are just simply called on the field marked as `#[destination]`.
/// - `#[rx_delegate_subscription_like_to_destination]` (optional): Opts into
///   the trivial implementation of `SubscriptionLike` where the traits methods
///   are just simply called on the field marked as `#[destination]`.
/// - `#[rx_delegate_observer_to_destination]` (optional): Opts into
///   the trivial implementation of `Observer` where the traits methods
///   are just simply called on the field marked as `#[destination]`.
/// - `#[rx_skip_unsubscribe_on_drop_impl]`: Skips the default
///   unsubscribe-on-drop implementation. Only use when the subscription
///   explicitly does NOT have to unsubscribe on drop, or you want to provide
///   your own implementation.
///
///   The default implementation:
///
///   ```text
///   fn drop(&mut self) {
///       if !self.is_closed() {
///           self.unsubscribe();
///       }
///   }
///   ```
#[proc_macro_derive(
	RxSubscriber,
	attributes(
		rx_in,
		rx_in_error,
		rx_does_not_upgrade_to_self,
		rx_upgrades_to,
		rx_delegate_teardown_collection,
		rx_delegate_subscription_like_to_destination,
		rx_delegate_observer_to_destination,
		rx_skip_unsubscribe_on_drop_impl,
		destination,
		teardown,
		_rx_core_traits_crate
	)
)]
pub fn subscriber_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl = impl_primary_category(&derive_input, primary_category_subscriber());
	let observer_input_impl = impl_observer_input(&derive_input);
	let subscriber_does_not_upgrade_to_self_impl =
		impl_subscriber_does_not_upgrade_to_self(&derive_input);
	let observer_upgrades_to_impl = impl_observer_upgrades_to(&derive_input);
	let delegate_teardown_collection_to_destination_impl =
		impl_delegate_teardown_collection(&derive_input);
	let delegate_subscription_like_to_destination_impl =
		impl_delegate_subscription_like_to_destination(&derive_input);
	let delegate_observer_to_destination_impl =
		impl_delegate_observer_to_destination(&derive_input);
	let skip_unsubscribe_on_drop_impl = impl_skip_unsubscribe_on_drop_impl(&derive_input);

	(quote! {
		#primary_category_impl

		#observer_input_impl

		#observer_upgrades_to_impl

		#subscriber_does_not_upgrade_to_self_impl

		#delegate_teardown_collection_to_destination_impl

		#delegate_subscription_like_to_destination_impl

		#delegate_observer_to_destination_impl

		#skip_unsubscribe_on_drop_impl
	})
	.into()
}

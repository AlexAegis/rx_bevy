use quote::quote;
use rx_core_macro_common::{
	derive_observer::{
		impl_does_not_upgrade_to_observer_subscriber, impl_observer_input,
		impl_observer_upgrades_to,
	},
	derive_primary_category::impl_primary_category,
};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_observer() -> Type {
	parse_quote! {
		PrimaryCategoryObserver
	}
}

/// # RxObserver
///
/// Helper macro to implement a few traits required for a observer.
///
/// ## Traits you still have to implement to get a observer
///
/// - `RxObserver`
///
/// ## Traits Implemented
///
/// - `WithPrimaryCategory`: Sets the associated type to
///   `PrimaryCategoryObserver`
/// - `ObserverInput`: Sets the associated type `In` to the value of the
///   `#[rx_in(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
///   sets the associated `InError` type to the value of the
///   `#[rx_in_error(...)]` attribute, or to `Never` if missing.
/// - `UpgradeableObserver`: By default. It implements `UpgradeableObserver` by
///   wrapping the subject into a `ObserverSubscriber`. This implementation can
///   be opted out with the `#[rx_does_not_upgrade_to_observer_subscriber]`
///   attribute to provide a manual implementation. Other preset
///   implementations can be used with the `#[rx_upgrades_to(...)]` attribute.
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_in(...)]` (optional, default: `Never`): Defines the input type of
///   the subscriber
/// - `#[rx_in_error(...)]` (optional, default: `Never`): Defines the input
///   error type of the subscriber
/// - `#[rx_does_not_upgrade_to_observer_subscriber]` (optional): Opts out the
///   default `UpgradeableObserver` implementation which just wraps the
///   `Subject` in a `ObserverSubscriber` when used as a destination for an
///   `Observable` to prevent upstream from unsubscribing the entire `Subject`.
/// - `#[rx_upgrades_to(...)]` (optional, accepts: `self`,
///   `observer_subscriber`): Defines a preset implementation for
///   `UpgradeableObserver`
///   - `self`: Upgraded version is itself, causing it to be unsubscribed
///     when upstream is unsubscribed when used as an observables destination.
///   - `observer_subscriber`: Upgraded version is itself wrapped in
///     `ObserverSubscriber`, causing it to **not** be unsubscribed when
///     upstream is unsubscribed when used as an observables destination.
#[proc_macro_derive(
	RxObserver,
	attributes(
		rx_in,
		rx_in_error,
		rx_does_not_upgrade_to_observer_subscriber,
		rx_upgrades_to,
		_rx_core_common_crate
	)
)]
pub fn subscriber_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl = impl_primary_category(&derive_input, primary_category_observer());
	let observer_input_impl = impl_observer_input(&derive_input);
	let observer_upgrades_to_impl = impl_observer_upgrades_to(&derive_input);
	let does_not_upgrade_to_observer_subscriber_impl =
		impl_does_not_upgrade_to_observer_subscriber(&derive_input);

	(quote! {
		#primary_category_impl

		#observer_input_impl

		#observer_upgrades_to_impl

		#does_not_upgrade_to_observer_subscriber_impl
	})
	.into()
}

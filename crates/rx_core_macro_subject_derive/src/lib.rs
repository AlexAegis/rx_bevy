use quote::quote;
use rx_core_macro_common::{
	impl_delegate_subscription_like_to_destination, impl_does_not_upgrade_to_observer_subscriber,
	impl_observable_output, impl_observer_input, impl_observer_upgrades_to, impl_primary_category,
};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_subject() -> Type {
	parse_quote! {
		rx_core_traits::PrimaryCategorySubject
	}
}

/// # RxSubject
///
/// Helper macro to implement a few traits required for a subject.
///
/// ## Traits you still have to implement to get a subject
///
/// - `Observable`
/// - `Observer`
/// - `SubscriptionLike` (unless using
///   `#[rx_delegate_subscription_like_to_destination]`)
///
/// ## Traits Implemented
///
/// - `WithPrimaryCategory`: Sets the associated type to
///   `PrimaryCategorySubject`
/// - `ObserverInput`: Sets the associated type `In` to the value of the
///   `#[rx_in(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
///   sets the associated `InError` type to the value of the
///   `#[rx_in_error(...)]` attribute, or to `Never` if missing.
/// - `ObservableOutput`: Sets the associated type `Out` to the value of the
///   `#[rx_out(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
///   sets the associated `OutError` type to the value of the
///   `#[rx_out_error(...)]` attribute, or to `Never` if missing.
/// - `UpgradeableObserver`: By default. It implements `UpgradeableObserver` by
///   wrapping the subject into a `ObserverSubscriber`. This implementation can
///   be opted out with the `#[rx_does_not_upgrade_to_observer_subscriber]` attribute to
///   provide a manual implementation. Other preset implementations can be
///   used with the `#[rx_upgrades_to(...)]` attribute.
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_in(...)]` (optional, default: `Never`): Defines the input type of
///   the subject
/// - `#[rx_in_error(...)]` (optional, default: `Never`): Defines the input
///   error type of the subject
/// - `#[rx_out(...)]` (optional, default: `Never`): Defines the output type of
///   the subject, usually it's the same as the input type
/// - `#[rx_out_error(...)]` (optional, default: `Never`): Defines the output
///   error type of the subject, usually it's the same as the input error type
/// - `#[rx_does_not_upgrade_to_observer_subscriber]` (optional): Opts out the default
///   `UpgradeableObserver` implementation which just wraps the `Subject` in a
///   `ObserverSubscriber` when used as a destination for an `Observable` to
///   prevent upstream from unsubscribing the entire `Subject`.
/// - `#[rx_upgrades_to(...)]` (optional, accepts: `self`,
///   `observer_subscriber`): Defines a preset implementation for
///   `UpgradeableObserver`
///   - `self`: Upgraded version is itself, causing it to be unsubscribed
///     when upstream is unsubscribed when used as an observables destination.
///   - `observer_subscriber`: Upgraded version is itself wrapped in
///     `ObserverSubscriber`, causing it to **not** be unsubscribed when
///     upstream is unsubscribed when used as an observables destination.
/// - `#[rx_delegate_subscription_like_to_destination]` (optional): Opts into
///   the trivial implementation of `SubscriptionLike` where the traits methods
///   are just simply called on the field marked as `#[destination]`.
#[proc_macro_derive(
	RxSubject,
	attributes(
		rx_in,
		rx_in_error,
		rx_out,
		rx_out_error,
		rx_does_not_upgrade_to_observer_subscriber,
		rx_upgrades_to,
		rx_delegate_subscription_like_to_destination,
		destination
	)
)]
pub fn subject_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl = impl_primary_category(&derive_input, primary_category_subject());
	let observable_output_impl = impl_observable_output(&derive_input);
	let observer_input_impl = impl_observer_input(&derive_input);
	let observer_upgrades_to_impl = impl_observer_upgrades_to(&derive_input);
	let does_not_upgrade_to_observer_subscriber_impl =
		impl_does_not_upgrade_to_observer_subscriber(&derive_input);
	let delegate_subscription_like_to_destination_impl =
		impl_delegate_subscription_like_to_destination(&derive_input);

	(quote! {
		#primary_category_impl

		#observable_output_impl

		#observer_input_impl

		#observer_upgrades_to_impl

		#does_not_upgrade_to_observer_subscriber_impl

		#delegate_subscription_like_to_destination_impl
	})
	.into()
}

use quote::quote;
use rx_core_macro_common::{
	impl_observable_output, impl_observer_input, impl_primary_category,
	impl_subject_does_not_upgrade_to_detached, impl_with_subscription_context,
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
/// ## Traits you have to implement
///
/// - `Observable`
/// - `Observer`
/// - `SubscriptionLike`
///
/// ## Traits Implemented
///
/// - `WithPrimaryCategory`: Sets the associated type to `PrimaryCategorySubject`
/// - `WithSubscriptionContext`: Sets the associated type to the values of the
///   `#[rx_context(...)]` attribute
/// - `ObserverInput`: Sets the associated type `In` to the value of the
///   `#[rx_in(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
///   sets the associated `InError` type to the value of the
///   `#[rx_in_error(...)]` attribute, or to `Never` if missing.
/// - `ObservableOutput`: Sets the associated type `Out` to the value of the
///   `#[rx_out(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
///   sets the associated `OutError` type to the value of the
///   `#[rx_out_error(...)]` attribute, or to `Never` if missing.
/// - `UpgradeableObserver`: Implements `UpgradeableObserver` by wrapping the
///   subject into a `DetachedSubscriber`. This implementation can be opted out
///   with the `#[rx_does_not_upgrade_to_detached]` attribute.
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_in(...)]` (optional, default: `Never`): Defines the input type of the
///   subject
/// - `#[rx_in_error(...)]` (optional, default: `Never`): Defines the input error
///   type of the subject
/// - `#[rx_out(...)]` (optional, default: `Never`): Defines the output type of
///   the subject, usually it's the same as the input type
/// - `#[rx_out_error(...)]` (optional, default: `Never`): Defines the output
///   error type of the subject, usually it's the same as the input error type
/// - `#[rx_context(...)]`: Defines the Context this subject is compatible with
/// - `#[rx_does_not_upgrade_to_detached]` (optional): Opts out the default
///   `UpgradeableObserver` implementation which just wraps the `Subject` in a
///   `DetachedSubscriber` when used as a destination for an `Observable` to
///   prevent upstream from unsubscribing the entire `Subject`.
#[proc_macro_derive(
	RxSubject,
	attributes(
		rx_in,
		rx_in_error,
		rx_out,
		rx_out_error,
		rx_context,
		rx_does_not_upgrade_to_detached
	)
)]
pub fn subject_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl = impl_primary_category(&derive_input, primary_category_subject());
	let observable_output_impl = impl_observable_output(&derive_input);
	let observer_input_impl = impl_observer_input(&derive_input);
	let with_subscription_context_impl = impl_with_subscription_context(&derive_input);
	let subject_does_not_upgrade_to_detached_impl =
		impl_subject_does_not_upgrade_to_detached(&derive_input);

	(quote! {
		#primary_category_impl

		#observable_output_impl

		#observer_input_impl

		#with_subscription_context_impl

		#subject_does_not_upgrade_to_detached_impl
	})
	.into()
}

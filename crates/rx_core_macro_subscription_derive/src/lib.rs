use quote::quote;
use rx_core_macro_common::{impl_primary_category, impl_with_subscription_context};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_subscription() -> Type {
	parse_quote! {
		rx_core_traits::PrimaryCategorySubscription
	}
}

/// # RxSubscription
///
/// Helper macro to implement a few traits required for a subscription.
///
/// ## Traits you still have to implement to get a subscriber
///
/// - `SubscriptionLike`
/// - `TeardownCollection`
/// - `Tickable`
/// - `Drop`
///
/// ## Traits Implemented
///
/// - `WithPrimaryCategory`: Sets the associated type to
///   `PrimaryCategorySubscription`
/// - `WithSubscriptionContext`: Sets the associated type to the values of the
///   `#[rx_context(...)]` attribute
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_context(...)]`: Defines the Context this subscriber is compatible with
#[proc_macro_derive(RxSubscription, attributes(rx_context))]
pub fn subscription_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl =
		impl_primary_category(&derive_input, primary_category_subscription());
	let with_subscription_context_impl = impl_with_subscription_context(&derive_input);

	(quote! {
		#primary_category_impl

		#with_subscription_context_impl
	})
	.into()
}

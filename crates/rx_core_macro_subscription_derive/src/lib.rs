use quote::quote;
use rx_core_macro_common::{
	impl_delegate_teardown_collection_to_destination, impl_delegate_tickable_to_destination,
	impl_primary_category, impl_skip_unsubscribe_on_drop_impl, impl_with_subscription_context,
};
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
/// - `Tickable` (unless using `#[rx_delegate_tickable_to_destination]`)
/// - `TeardownCollection` (unless using
///   `#[rx_delegate_teardown_collection_to_destination]`)
///
/// ## Traits Implemented
///
/// - `WithPrimaryCategory`: Sets the associated type to
///   `PrimaryCategorySubscription`
/// - `WithSubscriptionContext`: Sets the associated type to the values
///   of the `#[rx_context(...)]` attribute
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_context(...)]`: Defines the Context this subscriber is compatible
///   with
/// - `#[rx_delegate_tickable_to_destination]` (optional): Opts into
///   the trivial implementation of `Tickable` where the traits methods
///   are just simply called on the field marked as `#[destination]`.
/// - `#[rx_skip_unsubscribe_on_drop_impl]`: Skips the default
///   unsubscribe-on-drop implementation that will panic for
///   DropUnsafeSubscriptionContexts if they were not closed before dropped.
///   This is the default, expected behavior but some Subscriptions may ensure
///   correct operation differently.
#[proc_macro_derive(
	RxSubscription,
	attributes(
		rx_context,
		rx_delegate_tickable_to_destination,
		rx_delegate_teardown_collection_to_destination,
		rx_skip_unsubscribe_on_drop_impl,
		destination
	)
)]
pub fn subscription_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl =
		impl_primary_category(&derive_input, primary_category_subscription());
	let with_subscription_context_impl = impl_with_subscription_context(&derive_input);
	let delegate_tickable_to_destination_impl =
		impl_delegate_tickable_to_destination(&derive_input);
	let delegate_teardown_collection_to_destination_impl =
		impl_delegate_teardown_collection_to_destination(&derive_input);
	let skip_unsubscribe_on_drop_impl = impl_skip_unsubscribe_on_drop_impl(&derive_input);

	(quote! {
		#primary_category_impl

		#with_subscription_context_impl

		#delegate_tickable_to_destination_impl

		#delegate_teardown_collection_to_destination_impl

		#skip_unsubscribe_on_drop_impl
	})
	.into()
}

use quote::quote;
use rx_core_macro_common::{
	derive_primary_category::impl_primary_category,
	derive_subscription::{
		impl_delegate_subscription_like_to_destination, impl_skip_unsubscribe_on_drop_impl,
	},
	derive_teardown_collection::impl_delegate_teardown_collection,
};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_subscription() -> Type {
	parse_quote! {
		PrimaryCategorySubscription
	}
}

/// # RxSubscription
///
/// Helper macro to implement a few traits required for a subscription.
///
/// ## Traits you still have to implement to get a subscriber
///
/// - `SubscriptionLike` (unless using
///   `#[rx_delegate_subscription_like_to_destination]`)
/// - `TeardownCollection` (unless using `#[rx_delegate_teardown_collection]`)
///
/// ## Traits Implemented
///
/// - `WithPrimaryCategory`: Sets the associated type to
///   `PrimaryCategorySubscription`
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_delegate_teardown_collection]`: Implements `add_teardown`
///
///   The default implementation is:
///
///   ```text
///   fn add_teardown(&mut self, teardown: Teardown) {
///       if !self.is_closed() {
///           self.(#[teardown] or if missing, #[destination]).add_teardown(teardown);
///       } else {
///           teardown.execute();
///       }
///   }
///   ```
///
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
	RxSubscription,
	attributes(
		rx_delegate_teardown_collection,
		rx_skip_unsubscribe_on_drop_impl,
		rx_delegate_subscription_like_to_destination,
		destination,
		teardown,
		_rx_core_common_crate
	)
)]
pub fn subscription_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl =
		impl_primary_category(&derive_input, primary_category_subscription());
	let delegate_teardown_collection_to_destination_impl =
		impl_delegate_teardown_collection(&derive_input);
	let delegate_subscription_like_to_destination_impl =
		impl_delegate_subscription_like_to_destination(&derive_input);
	let skip_unsubscribe_on_drop_impl = impl_skip_unsubscribe_on_drop_impl(&derive_input);

	(quote! {
		#primary_category_impl

		#delegate_teardown_collection_to_destination_impl

		#delegate_subscription_like_to_destination_impl

		#skip_unsubscribe_on_drop_impl
	})
	.into()
}

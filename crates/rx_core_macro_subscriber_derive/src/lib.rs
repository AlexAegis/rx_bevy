use quote::quote;
use rx_core_macro_common::{
	impl_delegate_observer_to_destination, impl_delegate_subscription_like_to_destination,
	impl_delegate_teardown_collection_to_destination, impl_delegate_tickable_to_destination,
	impl_observable_output, impl_observer_input, impl_primary_category,
	impl_subscriber_does_not_upgrade_to_self, impl_with_subscription_context,
};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_subscriber() -> Type {
	parse_quote! {
		rx_core_traits::PrimaryCategorySubscriber
	}
}

/// # RxSubscriber
#[proc_macro_derive(
	RxSubscriber,
	attributes(
		rx_in,
		rx_in_error,
		rx_out,
		rx_out_error,
		rx_context,
		rx_does_not_upgrade_to_self,
		rx_delegate_tickable_to_destination,
		rx_delegate_teardown_collection_to_destination,
		rx_delegate_subscription_like_to_destination,
		rx_delegate_observer_to_destination,
		destination
	)
)]
pub fn subscriber_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl = impl_primary_category(&derive_input, primary_category_subscriber());
	let observable_output_impl = impl_observable_output(&derive_input);
	let observer_input_impl = impl_observer_input(&derive_input);
	let with_subscription_context_impl = impl_with_subscription_context(&derive_input);
	let subscriber_does_not_upgrade_to_self_impl =
		impl_subscriber_does_not_upgrade_to_self(&derive_input);
	let delegate_tickable_to_destination_impl =
		impl_delegate_tickable_to_destination(&derive_input);
	let delegate_teardown_collection_to_destination_impl =
		impl_delegate_teardown_collection_to_destination(&derive_input);
	let delegate_subscription_like_to_destination_impl =
		impl_delegate_subscription_like_to_destination(&derive_input);
	let delegate_observer_to_destination_impl =
		impl_delegate_observer_to_destination(&derive_input);

	(quote! {
		#primary_category_impl

		#observable_output_impl

		#observer_input_impl

		#with_subscription_context_impl

		#subscriber_does_not_upgrade_to_self_impl

		#delegate_tickable_to_destination_impl

		#delegate_teardown_collection_to_destination_impl

		#delegate_subscription_like_to_destination_impl

		#delegate_observer_to_destination_impl
	})
	.into()
}

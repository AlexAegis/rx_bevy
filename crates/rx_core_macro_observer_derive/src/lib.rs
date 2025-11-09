use quote::quote;
use rx_core_macro_common::{
	impl_observer_input, impl_observer_upgrades_to_detached, impl_observer_upgrades_to_self,
	impl_primary_category, impl_with_subscription_context,
};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_observer() -> Type {
	parse_quote! {
		rx_core_traits::PrimaryCategoryObserver
	}
}

#[proc_macro_derive(
	RxObserver,
	attributes(
		r#in,
		rx_in_error,
		rx_context,
		rx_upgrades_to_self,
		rx_upgrades_to_detached
	)
)]
pub fn subscriber_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl = impl_primary_category(&derive_input, primary_category_observer());
	let observer_input_impl = impl_observer_input(&derive_input);
	let with_subscription_context_impl = impl_with_subscription_context(&derive_input);
	let observer_upgrades_to_self_impl = impl_observer_upgrades_to_self(&derive_input);
	let observer_upgrades_to_detached_impl = impl_observer_upgrades_to_detached(&derive_input);

	(quote! {
		#primary_category_impl

		#observer_input_impl

		#with_subscription_context_impl

		#observer_upgrades_to_self_impl

		#observer_upgrades_to_detached_impl
	})
	.into()
}

use quote::quote;
use rx_core_macro_common::{
	impl_observable_output, impl_primary_category, impl_with_subscription_context,
};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_observable() -> Type {
	parse_quote! {
		rx_core_traits::PrimaryCategoryObservable
	}
}

#[proc_macro_derive(RxObservable, attributes(rx_out, rx_out_error, rx_context))]
pub fn observable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl = impl_primary_category(&derive_input, primary_category_observable());
	let observable_output_impl = impl_observable_output(&derive_input);
	let with_subscription_context_impl = impl_with_subscription_context(&derive_input);

	(quote! {
		#primary_category_impl

		#observable_output_impl

		#with_subscription_context_impl
	})
	.into()
}

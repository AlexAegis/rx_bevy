use quote::quote;
use rx_core_macro_common::{impl_observable_output, impl_observer_input, impl_primary_category};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_operator() -> Type {
	parse_quote! {
		rx_core_traits::PrimaryCategoryOperator
	}
}

#[proc_macro_derive(RxOperator, attributes(rx_in, rx_in_error, rx_out, rx_out_error))]
pub fn operator_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl = impl_primary_category(&derive_input, primary_category_operator());
	let observable_output_impl = impl_observable_output(&derive_input);
	let observer_input_impl = impl_observer_input(&derive_input);

	(quote! {
		#primary_category_impl

		#observable_output_impl

		#observer_input_impl
	})
	.into()
}

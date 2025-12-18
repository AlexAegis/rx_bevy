use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::{find_attribute, get_rx_core_traits_crate, never_type, read_attribute_type};

pub fn impl_observable_output(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let out_type = find_attribute(&derive_input.attrs, "rx_out")
		.map(read_attribute_type)
		.unwrap_or(never_type(derive_input));
	let out_error_type = find_attribute(&derive_input.attrs, "rx_out_error")
		.map(read_attribute_type)
		.unwrap_or(never_type(derive_input));

	let _rx_core_traits_crate = get_rx_core_traits_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_traits_crate::ObservableOutput for #ident #ty_generics #where_clause {
			type Out = #out_type;
			type OutError = #out_error_type;
		}
	}
}

#[cfg(test)]
mod test {
	use quote::quote;
	use syn::{DeriveInput, parse_quote};

	use crate::derive_observable::impl_observable_output;

	#[test]
	fn should_default_to_never() {
		let input: DeriveInput = parse_quote! { struct Foo; };
		let tokens = impl_observable_output(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl rx_core_traits::ObservableOutput for Foo }.to_string()));
		assert!(s.contains(&quote! { type Out = rx_core_traits::Never; }.to_string()));
		assert!(s.contains(&quote! { type OutError = rx_core_traits::Never; }.to_string()));
	}

	#[test]
	fn should_respect_crate_override() {
		let input: DeriveInput = parse_quote! {
			#[_rx_core_traits_crate(crate)]
			struct Foo;
		};
		let tokens = impl_observable_output(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl crate::ObservableOutput for Foo }.to_string()));
		assert!(s.contains(&quote! { type Out = crate::Never; }.to_string()));
		assert!(s.contains(&quote! { type OutError = crate::Never; }.to_string()));
	}

	#[test]
	fn should_use_the_specified_types_as_output_types() {
		let input: DeriveInput = parse_quote! {
			#[rx_out(String)]
			#[rx_out_error(u8)]
			struct Foo;
		};
		let tokens = impl_observable_output(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { type Out = String; }.to_string()));
		assert!(s.contains(&quote! { type OutError = u8; }.to_string()));
	}

	#[test]
	fn should_use_the_specified_types_as_output_types_and_default_non_specified_ones() {
		let input: DeriveInput = parse_quote! {
			#[rx_out(i32)]
			struct Foo;
		};
		let tokens = super::impl_observable_output(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { type Out = i32; }.to_string()));
		assert!(s.contains(&quote! { type OutError = rx_core_traits::Never; }.to_string()));
	}
}

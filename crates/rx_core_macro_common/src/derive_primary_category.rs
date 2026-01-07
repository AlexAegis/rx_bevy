use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type};

use crate::helpers::get_rx_core_common_crate;

pub fn impl_primary_category(derive_input: &DeriveInput, primary_category: Type) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let _rx_core_common_crate = get_rx_core_common_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_common_crate::WithPrimaryCategory for #ident #ty_generics #where_clause {
			type PrimaryCategory = #_rx_core_common_crate::#primary_category;
		}
	}
}

#[cfg(test)]
mod test {
	use quote::quote;
	use syn::{DeriveInput, parse_quote};

	use crate::derive_primary_category::impl_primary_category;

	#[test]
	fn should_default_to_rx_core_common() {
		let input: DeriveInput = parse_quote! {
			struct Foo;
		};
		let tokens = impl_primary_category(&input, parse_quote! { PrimaryCategoryCustom });
		let s = tokens.to_string();
		assert!(
			s.contains(&quote! { impl rx_core_common::WithPrimaryCategory for Foo }.to_string())
		);
		assert!(s.contains(
			&quote! { type PrimaryCategory = rx_core_common::PrimaryCategoryCustom; }.to_string()
		));
	}

	#[test]
	fn should_respect_crate_override() {
		let input: DeriveInput = parse_quote! {
			#[_rx_core_common_crate(crate)]
			struct Foo;
		};
		let tokens = impl_primary_category(&input, parse_quote! { PrimaryCategoryCustom });
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl crate::WithPrimaryCategory for Foo }.to_string()));
		assert!(s.contains(
			&quote! { type PrimaryCategory = crate::PrimaryCategoryCustom; }.to_string()
		));
	}
}

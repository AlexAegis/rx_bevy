use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::{find_attribute, get_rx_core_traits_crate, read_attribute_type, unit_type};

pub fn impl_with_work_context_provider(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let context_type = find_attribute(&derive_input.attrs, "rx_context")
		.map(read_attribute_type)
		.unwrap_or(unit_type());

	let _rx_core_traits_crate = get_rx_core_traits_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_traits_crate::WithWorkContextProvider for #ident #ty_generics #where_clause {
			type WorkContextProvider = #context_type;
		}
	}
}

#[cfg(test)]
mod test {

	use quote::quote;
	use syn::{DeriveInput, parse_quote};

	use crate::derive_with_context_provider::impl_with_work_context_provider;

	#[test]
	fn should_default_to_unit() {
		let input: DeriveInput = parse_quote! { struct Foo; };
		let tokens = impl_with_work_context_provider(&input);
		let s = tokens.to_string();
		assert!(s.contains(
			&quote! { impl rx_core_traits::WithWorkContextProvider for Foo }.to_string()
		));
		assert!(s.contains(&quote! { type WorkContextProvider = (); }.to_string()));
	}

	#[test]
	fn should_respect_attr() {
		let input: DeriveInput = parse_quote! {
			#[rx_context(ContextType)]
			struct Foo;
		};
		let tokens = impl_with_work_context_provider(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { type WorkContextProvider = ContextType; }.to_string()));
	}
}

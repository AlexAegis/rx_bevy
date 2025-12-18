use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::{find_attribute, get_rx_core_traits_crate, never_type, read_attribute_type};

pub fn impl_with_task_input_output(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let tick_input_type = find_attribute(&derive_input.attrs, "rx_tick")
		.map(read_attribute_type)
		.unwrap_or(never_type(derive_input));

	let _rx_core_traits_crate = get_rx_core_traits_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_traits_crate::WithTaskInputOutput for #ident #ty_generics #where_clause {
			type Tick = #tick_input_type;
		}
	}
}

#[cfg(test)]
mod test {

	use quote::quote;
	use syn::{DeriveInput, parse_quote};

	use crate::derive_task::impl_with_task_input_output;

	#[test]
	fn should_default_to_never() {
		let input: DeriveInput = parse_quote! { struct Foo; };
		let tokens = impl_with_task_input_output(&input);
		let s = tokens.to_string();
		assert!(
			s.contains(&quote! { impl rx_core_traits::WithTaskInputOutput for Foo }.to_string())
		);
		assert!(s.contains(&quote! { type Tick = rx_core_traits::Never; }.to_string()));
	}

	#[test]
	fn should_respect_the_set_value() {
		let input: DeriveInput = parse_quote! {
			#[rx_tick(TickType)]
			struct Foo;
		};
		let tokens = impl_with_task_input_output(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { type Tick = TickType; }.to_string()));
	}
}

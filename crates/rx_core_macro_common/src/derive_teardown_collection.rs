use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::{find_attribute, find_field_ident_with_attribute, get_rx_core_common_crate};

pub fn impl_delegate_teardown_collection(derive_input: &DeriveInput) -> Option<TokenStream> {
	let rx_delegate_teardown_collection =
		find_attribute(&derive_input.attrs, "rx_delegate_teardown_collection").is_some();

	if rx_delegate_teardown_collection {
		Some(impl_delegate_teardown_collection_inner(derive_input))
	} else {
		None
	}
}

fn impl_delegate_teardown_collection_inner(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let teardown_field = find_field_ident_with_attribute(
		derive_input,
		"teardown",
		Some("destination"),
		"rx_delegate_teardown_collection",
		"TeardownCollection",
	)
	.or_else(|_| {
		find_field_ident_with_attribute(
			derive_input,
			"destination",
			None,
			"rx_delegate_teardown_collection",
			"TeardownCollection",
		)
	})
	.unwrap_or_else(|e| panic!("{}", e));

	let _rx_core_common_crate = get_rx_core_common_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_common_crate::TeardownCollection for #ident #ty_generics #where_clause {
			#[inline]
			fn add_teardown(
				&mut self,
				teardown: #_rx_core_common_crate::Teardown
			) {
				#_rx_core_common_crate::TeardownCollection::add_teardown(&mut self.#teardown_field, teardown);
			}
		}
	}
}

#[cfg(test)]
mod test {
	use quote::quote;

	use syn::{DeriveInput, parse_quote};

	use crate::{
		derive_teardown_collection::impl_delegate_teardown_collection, helpers::mute_panic,
	};

	#[test]
	fn should_prioritize_the_teardown_field() {
		let input: DeriveInput = parse_quote! {
			#[rx_delegate_teardown_collection]
			struct Foo {
				#[teardown]
				teardown: Dummy,
				#[destination]
				not_me: Destination,
			}
		};
		let tokens = impl_delegate_teardown_collection(&input).unwrap();
		let s = tokens.to_string();
		assert!(
			s.contains(&quote! { impl rx_core_common::TeardownCollection for Foo }.to_string())
		);
		assert!(
			s.contains(
				&quote! {
					#[inline]
					fn add_teardown(
						&mut self,
						teardown: rx_core_common::Teardown
					) {
						rx_core_common::TeardownCollection::add_teardown(&mut self.teardown, teardown);
					}
				}
				.to_string()
			)
		);
	}

	#[test]
	fn should_fall_back_to_using_the_destination_field() {
		let input: DeriveInput = parse_quote! {
			#[rx_delegate_teardown_collection]
			struct Foo {
				#[destination]
				destination: Dummy,
			}
		};
		let tokens = impl_delegate_teardown_collection(&input).unwrap();
		let s = tokens.to_string();
		assert!(
			s.contains(
				&quote! {
					#[inline]
					fn add_teardown(
						&mut self,
						teardown: rx_core_common::Teardown
					) {
						rx_core_common::TeardownCollection::add_teardown(&mut self.destination, teardown);
					}
				}
				.to_string()
			)
		);
	}

	#[test]
	#[should_panic]
	fn should_panic_when_missing_both_fields() {
		let input: DeriveInput = parse_quote! {
			#[rx_delegate_teardown_collection]
			struct Foo;
		};

		mute_panic(|| impl_delegate_teardown_collection(&input).unwrap());
	}

	#[test]
	fn should_return_none_when_not_requested() {
		let input: DeriveInput = parse_quote! { struct Foo; };
		assert!(impl_delegate_teardown_collection(&input).is_none());
	}
}

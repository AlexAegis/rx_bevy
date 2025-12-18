use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::{find_attribute, find_field_ident_with_attribute, get_rx_core_traits_crate};

pub fn impl_delegate_subscription_like_to_destination(
	derive_input: &DeriveInput,
) -> Option<TokenStream> {
	let rx_delegate_subscription_like_to_destination = find_attribute(
		&derive_input.attrs,
		"rx_delegate_subscription_like_to_destination",
	)
	.is_some();

	if rx_delegate_subscription_like_to_destination {
		Some(impl_delegate_subscription_like_to_destination_inner(
			derive_input,
		))
	} else {
		None
	}
}

fn impl_delegate_subscription_like_to_destination_inner(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let teardown_field = find_field_ident_with_attribute(
		derive_input,
		"teardown",
		Some("destination"),
		"rx_delegate_subscription_like_to_destination",
		"SubscriptionLike",
	)
	.unwrap_or_else(|e| panic!("{}", e));

	let destination_field = find_field_ident_with_attribute(
		derive_input,
		"destination",
		None,
		"rx_delegate_subscription_like_to_destination",
		"SubscriptionLike",
	)
	.unwrap_or_else(|e| panic!("{}", e));

	let unsubscribe_impl = if teardown_field != destination_field {
		quote! {
			self.#teardown_field.unsubscribe();
			self.#destination_field.unsubscribe();
		}
	} else {
		quote! {
			self.#destination_field.unsubscribe();
		}
	};

	let _rx_core_traits_crate = get_rx_core_traits_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_traits_crate::SubscriptionLike for #ident #ty_generics #where_clause {
			#[inline]
			fn is_closed(&self) -> bool {
				self.#destination_field.is_closed()
			}

			#[inline]
			fn unsubscribe(&mut self) {
				#unsubscribe_impl
			}
		}
	}
}

/// Implements automatic unsubscribe on drop
fn impl_unsubscribe_on_drop(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let _rx_core_traits_crate = get_rx_core_traits_crate(derive_input);

	quote! {
		impl #impl_generics Drop for #ident #ty_generics #where_clause {
			#[track_caller]
			fn drop(&mut self) {
				if !#_rx_core_traits_crate::SubscriptionLike::is_closed(self) {
					#_rx_core_traits_crate::SubscriptionLike::unsubscribe(self);
				}
			}
		}
	}
}

pub fn impl_skip_unsubscribe_on_drop_impl(derive_input: &DeriveInput) -> Option<TokenStream> {
	let skip_unsubscribe_on_drop_impl =
		find_attribute(&derive_input.attrs, "rx_skip_unsubscribe_on_drop_impl").is_some();

	if skip_unsubscribe_on_drop_impl {
		None
	} else {
		Some(impl_unsubscribe_on_drop(derive_input))
	}
}

#[cfg(test)]
mod test {
	use quote::quote;
	use syn::{DeriveInput, parse_quote};

	use crate::derive_subscription::{
		impl_delegate_subscription_like_to_destination, impl_skip_unsubscribe_on_drop_impl,
	};

	#[test]
	fn should_use_both_the_destination_and_teardown_fields_when_both_present() {
		let input: DeriveInput = parse_quote! {
			#[rx_delegate_subscription_like_to_destination]
			struct Foo {
				#[teardown]
				teardown: Dummy,
				#[destination]
				destination: Dummy,
			}
		};
		let tokens = impl_delegate_subscription_like_to_destination(&input).unwrap();
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl rx_core_traits::SubscriptionLike for Foo }.to_string()));
		assert!(
			s.contains(
				&quote! {
					#[inline]
					fn is_closed(&self) -> bool {
						self.destination.is_closed()
					}

					#[inline]
					fn unsubscribe(&mut self) {
						self.teardown.unsubscribe();
						self.destination.unsubscribe();
					}
				}
				.to_string()
			)
		);
	}

	#[test]
	fn should_generate_one_unsubscribe_call_when_teardown_and_destination_is_the_same_field() {
		let input: DeriveInput = parse_quote! {
			#[rx_delegate_subscription_like_to_destination]
			struct Foo {
				#[teardown]
				#[destination]
				destination: Dummy,
			}
		};
		let tokens = impl_delegate_subscription_like_to_destination(&input).unwrap();
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl rx_core_traits::SubscriptionLike for Foo }.to_string()));
		assert!(
			s.contains(
				&quote! {
					#[inline]
					fn is_closed(&self) -> bool {
						self.destination.is_closed()
					}

					#[inline]
					fn unsubscribe(&mut self) {
						self.destination.unsubscribe();
					}
				}
				.to_string()
			)
		);
	}

	#[test]
	fn should_use_only_the_destination_field_when_there_is_no_teardown_field() {
		let input: DeriveInput = parse_quote! {
			#[rx_delegate_subscription_like_to_destination]
			struct Foo {
				#[destination]
				destination: Dummy,
			}
		};
		let tokens = impl_delegate_subscription_like_to_destination(&input).unwrap();
		let s = tokens.to_string();
		assert!(
			s.contains(
				&quote! {
					#[inline]
					fn is_closed(&self) -> bool {
						self.destination.is_closed()
					}

					#[inline]
					fn unsubscribe(&mut self) {
						self.destination.unsubscribe();
					}
				}
				.to_string()
			)
		);
		assert!(!s.contains(&quote! { self.teardown.unsubscribe(); }.to_string()));
	}

	#[test]
	fn should_return_none_when_not_requested() {
		let input: DeriveInput = parse_quote! { struct Foo { #[destination] destination: Dummy } };
		assert!(impl_delegate_subscription_like_to_destination(&input).is_none());
	}

	#[test]
	fn should_impl_drop_by_default() {
		let input: DeriveInput = parse_quote! { struct Foo; };
		let tokens = impl_skip_unsubscribe_on_drop_impl(&input).unwrap();
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl Drop for Foo }.to_string()));
		assert!(
			s.contains(
				&quote! {
					#[track_caller]
					fn drop(&mut self) {
						if !rx_core_traits::SubscriptionLike::is_closed(self) {
							rx_core_traits::SubscriptionLike::unsubscribe(self);
						}
					}
				}
				.to_string()
			)
		);
	}

	#[test]
	fn should_skip_when_opted_out() {
		let input: DeriveInput = parse_quote! {
			#[rx_skip_unsubscribe_on_drop_impl]
			struct Foo;
		};

		assert!(impl_skip_unsubscribe_on_drop_impl(&input).is_none());
	}
}

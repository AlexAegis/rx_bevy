use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	DeriveInput, Ident, Token,
	parse::{Parse, ParseStream},
};

use crate::helpers::{
	find_attribute, find_field_ident_with_attribute, get_rx_core_common_crate, never_type,
	read_attribute_type,
};

pub fn impl_observer_input(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let in_type = find_attribute(&derive_input.attrs, "rx_in")
		.map(read_attribute_type)
		.unwrap_or(never_type(derive_input));
	let in_error_type = find_attribute(&derive_input.attrs, "rx_in_error")
		.map(read_attribute_type)
		.unwrap_or(never_type(derive_input));

	let _rx_core_common_crate = get_rx_core_common_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_common_crate::ObserverInput for #ident #ty_generics #where_clause {
			type In = #in_type;
			type InError = #in_error_type;
		}
	}
}

fn impl_upgrades_to_self(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let _rx_core_common_crate = get_rx_core_common_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_common_crate::ObserverUpgradesToSelf for #ident #ty_generics #where_clause {
		}
	}
}

pub fn impl_subscriber_does_not_upgrade_to_self(derive_input: &DeriveInput) -> Option<TokenStream> {
	let does_not_upgrade_to_self_attribute =
		find_attribute(&derive_input.attrs, "rx_does_not_upgrade_to_self").is_some();

	if does_not_upgrade_to_self_attribute
		|| find_attribute(&derive_input.attrs, "rx_upgrades_to").is_some()
	{
		None
	} else {
		Some(impl_upgrades_to_self(derive_input))
	}
}

pub fn impl_does_not_upgrade_to_observer_subscriber(
	derive_input: &DeriveInput,
) -> Option<TokenStream> {
	let does_not_upgrade_to_observer_subscriber = find_attribute(
		&derive_input.attrs,
		"rx_does_not_upgrade_to_observer_subscriber",
	)
	.is_some();

	if does_not_upgrade_to_observer_subscriber
		|| find_attribute(&derive_input.attrs, "rx_upgrades_to").is_some()
	{
		None
	} else {
		Some(impl_upgrades_to_detached(derive_input))
	}
}

pub fn impl_delegate_observer_to_destination(derive_input: &DeriveInput) -> Option<TokenStream> {
	let rx_delegate_observer_to_destination =
		find_attribute(&derive_input.attrs, "rx_delegate_observer_to_destination").is_some();

	if rx_delegate_observer_to_destination {
		Some(impl_delegate_observer_to_destination_inner(derive_input))
	} else {
		None
	}
}

fn impl_delegate_observer_to_destination_inner(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let destination_field = find_field_ident_with_attribute(
		derive_input,
		"destination",
		None,
		"rx_delegate_observer_to_destination",
		"RxObserver",
	)
	.unwrap_or_else(|e| panic!("{}", e));

	let _rx_core_common_crate = get_rx_core_common_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_common_crate::RxObserver for #ident #ty_generics #where_clause {
			#[inline]
			fn next(
				&mut self,
				next: Self::In
			) {
				self.#destination_field.next(next);
			}

			#[inline]
			fn error(
				&mut self,
				error: Self::InError
			) {
				self.#destination_field.error(error);
			}

			#[inline]
			fn complete(&mut self) {
				self.#destination_field.complete();
			}
		}
	}
}

#[derive(Clone, Copy)]
enum ObserverUpgrades {
	ToSelf,
	ToObserverSubscriber,
}

impl Parse for ObserverUpgrades {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let is_self = input.parse::<Token![self]>().is_ok();
		if is_self {
			return Ok(ObserverUpgrades::ToSelf);
		};

		let ident = input.parse::<Ident>()?;

		if ident == "observer_subscriber" {
			Ok(ObserverUpgrades::ToObserverSubscriber)
		} else {
			Err(syn::Error::new(
				ident.span(),
				"invalid value for #[rx_upgrades_to(..)]: expected `self` or `observer_subscriber`",
			))
		}
	}
}

pub fn impl_observer_upgrades_to(derive_input: &DeriveInput) -> Option<TokenStream> {
	let upgrades_to = find_attribute(&derive_input.attrs, "rx_upgrades_to");

	upgrades_to.map(|upgrades_to| {
		let target: ObserverUpgrades = upgrades_to.parse_args().unwrap();

		match target {
			ObserverUpgrades::ToObserverSubscriber => impl_upgrades_to_detached(derive_input),
			ObserverUpgrades::ToSelf => impl_upgrades_to_self(derive_input),
		}
	})
}

fn impl_upgrades_to_detached(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let _rx_core_common_crate = get_rx_core_common_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_common_crate::UpgradeableObserver for #ident #ty_generics #where_clause {
			type Upgraded = #_rx_core_common_crate::ObserverSubscriber<Self>;

			fn upgrade(self) -> Self::Upgraded {
				#_rx_core_common_crate::ObserverSubscriber::new(self)
			}
		}
	}
}

#[cfg(test)]
mod test {
	use quote::quote;
	use syn::{DeriveInput, parse_quote};

	use crate::{
		derive_observer::{
			ObserverUpgrades, impl_delegate_observer_to_destination,
			impl_does_not_upgrade_to_observer_subscriber, impl_observer_input,
			impl_observer_upgrades_to, impl_subscriber_does_not_upgrade_to_self,
		},
		helpers::find_attribute,
	};

	#[test]
	fn should_default_to_never_for_input_types() {
		let input: DeriveInput = parse_quote! { struct Foo; };
		let tokens = impl_observer_input(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl rx_core_common::ObserverInput for Foo }.to_string()));
		assert!(s.contains(&quote! { type In = rx_core_common::Never; }.to_string()));
		assert!(s.contains(&quote! { type InError = rx_core_common::Never; }.to_string()));
	}

	#[test]
	fn should_respect_crate_override() {
		let input: DeriveInput = parse_quote! {
			#[_rx_core_common_crate(crate)]
			struct Foo;
		};
		let tokens = impl_observer_input(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl crate::ObserverInput for Foo }.to_string()));
		assert!(s.contains(&quote! { type In = crate::Never; }.to_string()));
		assert!(s.contains(&quote! { type InError = crate::Never; }.to_string()));
	}

	#[test]
	fn should_use_the_specified_types_as_input_types() {
		let input: DeriveInput = parse_quote! {
			#[rx_in(f32)]
			#[rx_in_error(bool)]
			struct Foo;
		};
		let tokens = impl_observer_input(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { type In = f32; }.to_string()));
		assert!(s.contains(&quote! { type InError = bool; }.to_string()));
	}

	#[test]
	fn should_be_able_to_impl_delegate_observer_to_destination() {
		let input: DeriveInput = parse_quote! {
			#[rx_delegate_observer_to_destination]
			struct Foo {
				#[destination]
				destination: Dummy,
			}
		};
		let tokens = impl_delegate_observer_to_destination(&input).unwrap();
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl rx_core_common::RxObserver for Foo }.to_string()));
		assert!(
			s.contains(
				&quote! {
					#[inline]
					fn next(
						&mut self,
						next: Self::In
					) {
						self.destination.next(next);
					}

					#[inline]
					fn error(
						&mut self,
						error: Self::InError
					) {
						self.destination.error(error);
					}

					#[inline]
					fn complete(&mut self) {
						self.destination.complete();
					}
				}
				.to_string()
			)
		);
	}

	#[test]
	fn should_not_impl_delegate_observer_to_destination_when_not_requested() {
		let input: DeriveInput = parse_quote! { struct Foo { #[destination] destination: Dummy } };
		assert!(impl_delegate_observer_to_destination(&input).is_none());
	}

	mod observer_updates {
		use crate::helpers::mute_panic;

		use super::*;

		#[test]
		#[should_panic]
		fn should_reject_unknown_types() {
			let input: DeriveInput = parse_quote! {
				#[rx_upgrades_to(bogus)]
				struct Foo;
			};

			let attr = find_attribute(&input.attrs, "rx_upgrades_to").unwrap();
			mute_panic(|| attr.parse_args::<ObserverUpgrades>().unwrap());
		}

		#[test]
		fn should_use_does_not_upgrade_to_observer_subscriber_by_default() {
			let input: DeriveInput = parse_quote! { struct Foo; };
			let tokens = impl_does_not_upgrade_to_observer_subscriber(&input).unwrap();
			let s = tokens.to_string();
			assert!(s.contains(
				&quote! { impl rx_core_common::UpgradeableObserver for Foo }.to_string()
			));
			assert!(
				s.contains(&quote! { rx_core_common::ObserverSubscriber::new(self) }.to_string())
			);
		}

		#[test]
		fn should_not_use_does_not_upgrade_to_observer_subscriber_when_opted_out() {
			let input: DeriveInput = parse_quote! {
				#[rx_does_not_upgrade_to_observer_subscriber]
				struct Foo;
			};

			assert!(impl_does_not_upgrade_to_observer_subscriber(&input).is_none());
		}

		#[test]
		fn should_be_able_to_impl_observer_upgrades_to_self_as_a_special_case() {
			let input: DeriveInput = parse_quote! {
				#[rx_upgrades_to(self)]
				struct Foo;
			};
			let tokens = impl_observer_upgrades_to(&input).unwrap();
			assert!(tokens.to_string().contains(
				&quote! { impl rx_core_common::ObserverUpgradesToSelf for Foo }.to_string()
			));
		}

		#[test]
		fn should_be_able_to_impl_observer_upgrades_to_observer_subscriber() {
			let input: DeriveInput = parse_quote! {
				#[rx_upgrades_to(observer_subscriber)]
				struct Foo;
			};
			let tokens = impl_observer_upgrades_to(&input).unwrap();
			let s = tokens.to_string();
			assert!(s.contains(
				&quote! { impl rx_core_common::UpgradeableObserver for Foo }.to_string()
			));
			assert!(
				s.contains(&quote! { rx_core_common::ObserverSubscriber::new(self) }.to_string())
			);
		}

		#[test]
		fn should_upgrade_to_self_by_default() {
			let input: DeriveInput = parse_quote! { struct Foo; };
			let tokens = impl_subscriber_does_not_upgrade_to_self(&input).unwrap();
			let s = tokens.to_string();
			assert!(s.contains(
				&quote! { impl rx_core_common::ObserverUpgradesToSelf for Foo }.to_string()
			));
		}

		#[test]
		fn should_not_impl_upgrade_to_self_when_opted_out() {
			let input: DeriveInput = parse_quote! {
				#[rx_does_not_upgrade_to_self]
				struct Foo;
			};

			assert!(impl_subscriber_does_not_upgrade_to_self(&input).is_none());
		}

		#[test]
		fn should_not_impl_upgrade_to_self_when_upgrading_to_something_specific() {
			let input: DeriveInput = parse_quote! {
				#[rx_upgrades_to(self)]
				struct Foo;
			};

			assert!(impl_subscriber_does_not_upgrade_to_self(&input).is_none());
		}
	}
}

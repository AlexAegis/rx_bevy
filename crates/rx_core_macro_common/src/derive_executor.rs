use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::{
	find_attribute, find_field_ident_with_attribute, get_rx_core_traits_crate, read_attribute_type,
};

pub fn impl_executor(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let scheduler_type = find_attribute(&derive_input.attrs, "rx_scheduler")
		.map(read_attribute_type)
		.expect("#[rx_scheduler(...)] must be defined with a Scheduler type!");

	let scheduler_field = find_field_ident_with_attribute(
		derive_input,
		"scheduler_handle",
		None,
		"rx_scheduler",
		"Scheduler",
	)
	.unwrap_or_else(|e| panic!("{}", e));

	let _rx_core_traits_crate = get_rx_core_traits_crate(derive_input);

	quote! {
		impl #impl_generics #_rx_core_traits_crate::WorkExecutor for #ident #ty_generics #where_clause {
			type Scheduler = #scheduler_type;

			#[inline]
			fn get_scheduler_handle(&self) -> #_rx_core_traits_crate::SchedulerHandle<Self::Scheduler> {
				self.#scheduler_field.get_scheduler_handle()
			}
		}
	}
}

#[cfg(test)]
mod test {
	use quote::quote;
	use syn::{DeriveInput, parse_quote};

	use crate::{derive_executor::impl_executor, helpers::mute_panic};

	#[test]
	fn should_generate_get_scheduler_handle() {
		let input: DeriveInput = parse_quote! {
			#[rx_scheduler(MyScheduler)]
			struct Foo {
				#[scheduler_handle]
				handle: Dummy,
			}
		};
		let tokens = impl_executor(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl rx_core_traits::WorkExecutor for Foo }.to_string()));
		assert!(s.contains(&quote! { type Scheduler = MyScheduler; }.to_string()));
		assert!(
			s.contains(
				&quote! {
					#[inline]
					fn get_scheduler_handle(&self) -> rx_core_traits::SchedulerHandle<Self::Scheduler> {
						self.handle.get_scheduler_handle()
					}
				}
				.to_string()
			)
		);
	}

	#[test]
	fn should_respect_the_custom_crate_name_and_generate_get_scheduler_handle() {
		let input: DeriveInput = parse_quote! {
			#[rx_scheduler(MyScheduler)]
			#[_rx_core_traits_crate(crate)]
			struct Foo {
				#[scheduler_handle]
				handle: Dummy,
			}
		};
		let tokens = impl_executor(&input);
		let s = tokens.to_string();
		assert!(s.contains(&quote! { impl crate::WorkExecutor for Foo }.to_string()));
		assert!(s.contains(&quote! { type Scheduler = MyScheduler; }.to_string()));
		assert!(
			s.contains(
				&quote! {
					#[inline]
					fn get_scheduler_handle(&self) -> crate::SchedulerHandle<Self::Scheduler> {
						self.handle.get_scheduler_handle()
					}
				}
				.to_string()
			)
		);
	}

	#[test]
	#[should_panic]
	fn should_panic_when_scheduler_handle_missing() {
		let input: DeriveInput = parse_quote! {
			#[rx_scheduler(MyScheduler)]
			struct Foo;
		};

		mute_panic(|| impl_executor(&input));
	}
}

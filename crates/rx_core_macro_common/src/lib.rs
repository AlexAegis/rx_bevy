use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Meta, Type, parse_quote, parse2};

pub fn find_attribute<'a>(attrs: &'a [Attribute], attribute_name: &str) -> Option<&'a Attribute> {
	attrs
		.iter()
		.find(|attr| attr.path().is_ident(attribute_name))
}

pub fn find_attribute_required<'a>(attrs: &'a [Attribute], attribute_name: &str) -> &'a Attribute {
	find_attribute(attrs, attribute_name).expect("Missing #[{attribute_name}] attribute!")
}

pub fn read_attribute_type(attr: &Attribute) -> Type {
	let attribute_name = attr.path().get_ident().expect("Missing attribute name!");
	match &attr.meta {
		Meta::List(list) => {
			let t: Result<Type, syn::Error> = parse2(list.tokens.clone());
			t.unwrap_or_else(|_| panic!("Missing value inside #[{attribute_name}(...)]"))
		}
		_ => panic!("#[{attribute_name}(...) attribute must only contain a single type!"),
	}
}

pub fn read_attribute_value(attrs: &[Attribute], attribute_name: &str) -> Option<TokenStream> {
	let name_path_attr = find_attribute(attrs, attribute_name);

	if let Some(attr) = name_path_attr {
		if let Meta::List(list) = &attr.meta {
			Some(list.tokens.clone())
		} else {
			panic!("#[{attribute_name}(...)] has the wrong type!");
		}
	} else {
		None
	}
}

pub fn never_type() -> Type {
	parse_quote! {
		rx_core_traits::Never
	}
}

pub fn impl_primary_category(derive_input: &DeriveInput, primary_category: Type) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	quote! {
		impl #impl_generics rx_core_traits::WithPrimaryCategory for #ident #ty_generics #where_clause {
			type PrimaryCategory = #primary_category;
		}
	}
}

pub fn impl_with_subscription_context(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let context_type = find_attribute(&derive_input.attrs, "rx_context")
		.map(read_attribute_type)
		.expect("Missing #[rx_context(...)] attribute!");

	quote! {
		impl #impl_generics rx_core_traits::WithSubscriptionContext for #ident #ty_generics #where_clause {
			type Context = #context_type;
		}
	}
}

pub fn impl_observable_output(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let out_type = find_attribute(&derive_input.attrs, "rx_out")
		.map(read_attribute_type)
		.unwrap_or(never_type());
	let out_error_type = find_attribute(&derive_input.attrs, "rx_out_error")
		.map(read_attribute_type)
		.unwrap_or(never_type());

	quote! {
		impl #impl_generics rx_core_traits::ObservableOutput for #ident #ty_generics #where_clause {
			type Out = #out_type;
			type OutError = #out_error_type;
		}
	}
}

pub fn impl_observer_input(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let in_type = find_attribute(&derive_input.attrs, "rx_in")
		.map(read_attribute_type)
		.unwrap_or(never_type());
	let in_error_type = find_attribute(&derive_input.attrs, "rx_in_error")
		.map(read_attribute_type)
		.unwrap_or(never_type());

	quote! {
		impl #impl_generics rx_core_traits::ObserverInput for #ident #ty_generics #where_clause {
			type In = #in_type;
			type InError = #in_error_type;
		}
	}
}

fn impl_upgrades_to_self(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	quote! {
		impl #impl_generics rx_core_traits::ObserverUpgradesToSelf for #ident #ty_generics #where_clause {
		}
	}
}

pub fn impl_subscriber_does_not_upgrade_to_self(derive_input: &DeriveInput) -> Option<TokenStream> {
	let does_not_upgrade_to_self_attribute =
		find_attribute(&derive_input.attrs, "rx_does_not_upgrade_to_self").is_some();

	if does_not_upgrade_to_self_attribute {
		None
	} else {
		Some(impl_upgrades_to_self(derive_input))
	}
}

pub fn impl_observer_upgrades_to_self(derive_input: &DeriveInput) -> Option<TokenStream> {
	let upgrades_to_self = find_attribute(&derive_input.attrs, "rx_upgrades_to_self").is_some();

	if upgrades_to_self {
		Some(impl_upgrades_to_self(derive_input))
	} else {
		None
	}
}

fn impl_upgrades_to_detached(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	quote! {
		impl #impl_generics rx_core_traits::UpgradeableObserver for #ident #ty_generics #where_clause {
			type Upgraded = rx_core_traits::DetachedSubscriber<Self>;

			fn upgrade(self) -> Self::Upgraded {
				rx_core_traits::DetachedSubscriber::new(self)
			}
		}
	}
}

pub fn impl_observer_upgrades_to_detached(derive_input: &DeriveInput) -> Option<TokenStream> {
	let upgrades_to_detached =
		find_attribute(&derive_input.attrs, "rx_upgrades_to_detached").is_some();

	if upgrades_to_detached {
		Some(impl_upgrades_to_detached(derive_input))
	} else {
		None
	}
}

pub fn impl_subject_does_not_upgrade_to_detached(
	derive_input: &DeriveInput,
) -> Option<TokenStream> {
	let does_not_upgrade_to_detached =
		find_attribute(&derive_input.attrs, "rx_does_not_upgrade_to_detached").is_some();

	if does_not_upgrade_to_detached {
		None
	} else {
		Some(impl_upgrades_to_detached(derive_input))
	}
}

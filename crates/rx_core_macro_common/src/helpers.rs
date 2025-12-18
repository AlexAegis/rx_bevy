use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Ident, Meta, Type, parse_quote, parse2};
use thiserror::Error;

pub(crate) fn get_rx_core_traits_crate(derive_input: &DeriveInput) -> TokenStream {
	read_attribute_value(&derive_input.attrs, "_rx_core_traits_crate")
		.unwrap_or(quote! { rx_core_traits })
}

pub fn find_attribute<'a>(attrs: &'a [Attribute], attribute_name: &str) -> Option<&'a Attribute> {
	attrs
		.iter()
		.find(|attr| attr.path().is_ident(attribute_name))
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

pub fn never_type(derive_input: &DeriveInput) -> Type {
	let _rx_core_traits_crate = get_rx_core_traits_crate(derive_input);

	parse_quote! {
		#_rx_core_traits_crate::Never
	}
}

pub fn unit_type() -> Type {
	parse_quote! {
		()
	}
}

#[derive(Error, Debug)]
pub(crate) enum FindFieldError {
	#[error("Only named fields are supported when using #[{trigger_attribute_name}]!")]
	UnsupportedField {
		trigger_attribute_name: &'static str,
	},
	#[error("Only structs are supported when using #[{trigger_attribute_name}]!")]
	UnsupportedData {
		trigger_attribute_name: &'static str,
	},
	#[error(
		"A field implementing `{required_trait_on_field}` must be marked with `#[{field_attribute_name}]`{fallback_field_name_error} when using #[{trigger_attribute_name}]!"
	)]
	FieldNotFound {
		required_trait_on_field: &'static str,
		field_attribute_name: &'static str,
		fallback_field_name_error: String,
		trigger_attribute_name: &'static str,
	},
}

pub(crate) fn find_field_ident_with_attribute(
	derive_input: &DeriveInput,
	field_attribute_name: &'static str,
	fallback_field_attribute_name: Option<&'static str>,
	trigger_attribute_name: &'static str,
	required_trait_on_field: &'static str,
) -> Result<Ident, FindFieldError> {
	let fields = match derive_input.data {
		syn::Data::Struct(ref data) => match &data.fields {
			syn::Fields::Named(fields) => Ok(&fields.named),
			_ => Err(FindFieldError::UnsupportedField {
				trigger_attribute_name,
			}),
		},
		_ => Err(FindFieldError::UnsupportedData {
			trigger_attribute_name,
		}),
	}?;

	let fallback_field_name_error = fallback_field_attribute_name
		.map(|fallback| format!(" or with `#[{fallback}]`"))
		.unwrap_or_default();

	fields
		.iter()
		.find(|field| {
			field.attrs.iter().any(|attr| {
				attr.path().is_ident(field_attribute_name)
					|| fallback_field_attribute_name
						.is_some_and(|fallback| attr.path().is_ident(fallback))
			})
		})
		.and_then(|field| field.ident.clone())
		.ok_or(FindFieldError::FieldNotFound {
			required_trait_on_field,
			field_attribute_name,
			fallback_field_name_error,
			trigger_attribute_name,
		})
}

pub fn mute_panic<R>(fun: impl FnOnce() -> R) -> R {
	let hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(|_| {}));
	let result = fun();
	std::panic::set_hook(hook);
	result
}

#[cfg(test)]
mod test {
	use super::*;
	use quote::quote;
	use syn::{DeriveInput, parse_quote};

	mod read_attribute_value {
		use super::*;

		#[test]
		fn should_extract_tokens() {
			let input: DeriveInput = parse_quote! {
				#[rx_out(u8)]
				struct Foo;
			};

			let tokens = read_attribute_value(&input.attrs, "rx_out").unwrap();

			assert_eq!(tokens.to_string(), "u8");
		}

		#[test]
		fn read_attribute_value_returns_none_when_absent() {
			let attrs: DeriveInput = parse_quote! {
				#[rx_out(u8)]
				struct Foo;
			};

			assert!(read_attribute_value(&attrs.attrs, "rx_in").is_none());
		}

		#[test]
		#[should_panic]
		fn read_attribute_value_panics_on_wrong_meta() {
			let attrs: DeriveInput = parse_quote! {
				#[rx_out]
				struct Foo;
			};

			mute_panic(|| read_attribute_value(&attrs.attrs, "rx_out"));
		}
	}

	mod find_attribute {
		use super::*;

		#[test]
		fn should_only_find_matches() {
			let input: DeriveInput = parse_quote! {
				#[foo]
				#[bar]
				struct Foo;
			};

			assert!(find_attribute(&input.attrs, "foo").is_some());
			assert!(find_attribute(&input.attrs, "zed").is_none());
		}

		#[test]
		fn should_extract_inner_type_with_read_attribute_type() {
			let input: DeriveInput = parse_quote! {
				#[rx_out(i32)]
				struct Foo;
			};

			let attr = find_attribute(&input.attrs, "rx_out").unwrap();
			let ty = read_attribute_type(attr);

			assert_eq!(quote! {#ty}.to_string(), "i32");
		}

		#[test]
		#[should_panic]
		fn should_panic_when_missing_value() {
			let input: DeriveInput = parse_quote! {
				#[rx_out]
				struct Foo;
			};

			let attr = find_attribute(&input.attrs, "rx_out").unwrap();
			mute_panic(|| read_attribute_type(attr));
		}
	}

	mod never_type {
		use super::*;

		#[test]
		fn should_default_to_rx_core_traits() {
			let input: DeriveInput = parse_quote! { struct Foo; };
			let ty = never_type(&input);

			assert_eq!(
				quote! {#ty}.to_string().replace(" ", ""),
				"rx_core_traits::Never"
			);
		}

		#[test]
		fn should_respect_custom_traits_crate() {
			let input: DeriveInput = parse_quote! {
				#[_rx_core_traits_crate(custom_traits)]
				struct Foo;
			};
			let ty = never_type(&input);

			assert_eq!(
				quote! {#ty}.to_string().replace(" ", ""),
				"custom_traits::Never"
			);
		}

		#[test]
		fn should_respect_crate_override() {
			let input: DeriveInput = parse_quote! {
				#[_rx_core_traits_crate(crate)]
				struct Foo;
			};
			let ty = never_type(&input);

			assert_eq!(quote! {#ty}.to_string().replace(" ", ""), "crate::Never");
		}
	}

	mod unit_type {
		use super::*;

		#[test]
		fn should_return_unit() {
			let ty = unit_type();

			assert_eq!(quote! {#ty}.to_string(), "()");
		}
	}

	mod find_field_ident_with_attribute {
		use super::*;

		#[test]
		#[should_panic]
		fn should_reject_non_structs() {
			let input: DeriveInput = parse_quote! {
				enum Foo { A }
			};

			mute_panic(|| {
				find_field_ident_with_attribute(
					&input,
					"destination",
					None,
					"rx_delegate_observer_to_destination",
					"Observer",
				)
				.unwrap()
			});
		}
	}
}

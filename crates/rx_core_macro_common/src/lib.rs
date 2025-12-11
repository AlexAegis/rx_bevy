use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	Attribute, DeriveInput, Error, Ident, Meta, Token, Type,
	parse::{Parse, ParseStream},
	parse_quote, parse2,
};

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

pub fn never_type() -> Type {
	parse_quote! {
		rx_core_traits::Never
	}
}

pub fn unit_type() -> Type {
	parse_quote! {
		()
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

#[derive(Clone, Copy)]
enum ObserverUpgrades {
	ToSelf,
	ToObserverSubscriber,
}

impl Parse for ObserverUpgrades {
	fn parse(input: ParseStream) -> Result<Self, Error> {
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

	quote! {
		impl #impl_generics rx_core_traits::UpgradeableObserver for #ident #ty_generics #where_clause {
			type Upgraded = rx_core_traits::ObserverSubscriber<Self>;

			fn upgrade(self) -> Self::Upgraded {
				rx_core_traits::ObserverSubscriber::new(self)
			}
		}
	}
}

fn find_field_ident_with_attribute(
	derive_input: &DeriveInput,
	field_attribute_name: &str,
	trigger_attribute_name: &str,
	required_trait_on_field: &str,
) -> Ident {
	let fields = match derive_input.data {
		syn::Data::Struct(ref data) => match &data.fields {
			syn::Fields::Named(fields) => &fields.named,
			_ => panic!("Only named fields are supported when using #[{trigger_attribute_name}]!"),
		},
		_ => {
			panic!("Only structs are supported when using #[{trigger_attribute_name}]!")
		}
	};

	fields
		.iter()
		.find(|field| {
			field
				.attrs
				.iter()
				.any(|attr| attr.path().is_ident(field_attribute_name))
		})
		.and_then(|field| field.ident.clone())
		.unwrap_or_else(||
			panic!("A field implementing `{required_trait_on_field}` must be marked with `#[{field_attribute_name}]` when using #[{trigger_attribute_name}]!"),
		)
}

pub fn impl_delegate_teardown_collection_to_destination(
	derive_input: &DeriveInput,
) -> Option<TokenStream> {
	let rx_delegate_teardown_collection_to_destination = find_attribute(
		&derive_input.attrs,
		"rx_delegate_teardown_collection_to_destination",
	)
	.is_some();

	if rx_delegate_teardown_collection_to_destination {
		Some(impl_delegate_teardown_collection_to_destination_inner(
			derive_input,
		))
	} else {
		None
	}
}

fn impl_delegate_teardown_collection_to_destination_inner(
	derive_input: &DeriveInput,
) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let destination_field = find_field_ident_with_attribute(
		derive_input,
		"destination",
		"rx_delegate_teardown_collection_to_destination",
		"TeardownCollection",
	);

	quote! {
		impl #impl_generics rx_core_traits::TeardownCollection for #ident #ty_generics #where_clause {
			#[inline]
			fn add_teardown(
				&mut self,
				teardown: rx_core_traits::Teardown
			) {
				self.#destination_field.add_teardown(teardown);
			}
		}
	}
}

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

	let destination_field = find_field_ident_with_attribute(
		derive_input,
		"destination",
		"rx_delegate_subscription_like_to_destination",
		"SubscriptionLike",
	);

	quote! {
		impl #impl_generics rx_core_traits::SubscriptionLike for #ident #ty_generics #where_clause {
			#[inline]
			fn is_closed(&self) -> bool {
				self.#destination_field.is_closed()
			}

			#[inline]
			fn unsubscribe(&mut self) {
				self.#destination_field.unsubscribe();
			}
		}
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
		"rx_delegate_observer_to_destination",
		"Observer",
	);

	quote! {
		impl #impl_generics rx_core_traits::Observer for #ident #ty_generics #where_clause {
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

/// Implements automatic unsubscribe on drop
fn impl_unsubscribe_on_drop(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	quote! {
		impl #impl_generics Drop for #ident #ty_generics #where_clause {
			#[track_caller]
			fn drop(&mut self) {
				if !rx_core_traits::SubscriptionLike::is_closed(self) {
					rx_core_traits::SubscriptionLike::unsubscribe(self);
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

pub fn impl_with_context_provider(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let context_type = find_attribute(&derive_input.attrs, "rx_context")
		.map(read_attribute_type)
		.unwrap_or(unit_type());

	quote! {
		impl #impl_generics rx_core_traits::WithContextProvider for #ident #ty_generics #where_clause {
			type ContextProvider = #context_type;
		}
	}
}

pub fn impl_with_task_input_output(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let tick_input_type = find_attribute(&derive_input.attrs, "rx_tick")
		.map(read_attribute_type)
		.unwrap_or(never_type());

	quote! {
		impl #impl_generics rx_core_traits::WithTaskInputOutput for #ident #ty_generics #where_clause {
			type Tick = #tick_input_type;
		}
	}
}

pub fn impl_executor(derive_input: &DeriveInput) -> TokenStream {
	let ident = derive_input.ident.clone();
	let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

	let scheduler_type = find_attribute(&derive_input.attrs, "rx_scheduler")
		.map(read_attribute_type)
		.expect("#[rx_scheduler(...)] must be defined with a Scheduler type!");

	let scheduler_field = find_field_ident_with_attribute(
		derive_input,
		"scheduler_handle",
		"rx_scheduler",
		"Scheduler",
	);

	quote! {
		impl #impl_generics rx_core_traits::TaskExecutor for #ident #ty_generics #where_clause {
			type Scheduler = #scheduler_type;

			#[inline]
			fn get_scheduler_handle(&self) -> SchedulerHandle<Self::Scheduler> {
				self.#scheduler_field.get_scheduler_handle()
			}
		}
	}
}

use quote::quote;
use rx_core_macro_common::{
	impl_observable_output, impl_primary_category, impl_with_subscription_context,
};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_observable() -> Type {
	parse_quote! {
		rx_core_traits::PrimaryCategoryObservable
	}
}

/// # RxObservable
///
/// Helper macro to implement a few traits required for an observable.
///
/// ## Traits you still have to implement to get an observable
///
/// - `Observable`
///
/// ## Traits Implemented
///
/// - `WithPrimaryCategory`: Sets the associated type to `PrimaryCategoryObservable`
/// - `WithSubscriptionContext`: Sets the associated type to the values of the
///   `#[rx_context(...)]` attribute
/// - `ObservableOutput`: Sets the associated type `Out` to the value of the
///   `#[rx_out(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
///   sets the associated `OutError` type to the value of the
///   `#[rx_out_error(...)]` attribute, or to `Never` if missing.
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_out(...)]` (optional, default: `Never`): Defines the output type of
///   the observable
/// - `#[rx_out_error(...)]` (optional, default: `Never`): Defines the output
///   error type of the observable
/// - `#[rx_context(...)]`: Defines the Context this observable is compatible
///   with
#[proc_macro_derive(RxObservable, attributes(rx_out, rx_out_error, rx_context))]
pub fn observable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let primary_category_impl = impl_primary_category(&derive_input, primary_category_observable());
	let observable_output_impl = impl_observable_output(&derive_input);
	let with_subscription_context_impl = impl_with_subscription_context(&derive_input);

	(quote! {
		#primary_category_impl

		#observable_output_impl

		#with_subscription_context_impl
	})
	.into()
}

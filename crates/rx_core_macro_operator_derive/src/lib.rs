use quote::quote;
use rx_core_macro_common::{impl_observable_output, impl_observer_input, impl_primary_category};
use syn::{DeriveInput, Type, parse_macro_input, parse_quote};

fn primary_category_operator() -> Type {
	parse_quote! {
		PrimaryCategoryOperator
	}
}

/// # RxOperator
///
/// Helper macro to implement a few traits required for an operator.
///
/// ## Traits you still have to implement to get an operator
///
/// - `Operator` or `ComposableOperator`
///
/// ## Traits Implemented
///
/// - `WithPrimaryCategory`: Sets the associated type to `PrimaryCategoryOperator`
/// - `ObserverInput`: Sets the associated type `In` to the value of the
///   `#[rx_in(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
///   sets the associated `InError` type to the value of the
///   `#[rx_in_error(...)]` attribute, or to `Never` if missing.
/// - `ObservableOutput`: Sets the associated type `Out` to the value of the
///   `#[rx_out(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
///   sets the associated `OutError` type to the value of the
///   `#[rx_out_error(...)]` attribute, or to `Never` if missing.
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_in(...)]` (optional, default: `Never`): Defines the input type of
///   the operator
/// - `#[rx_in_error(...)]` (optional, default: `Never`): Defines the input
///   error type of the operator
/// - `#[rx_out(...)]` (optional, default: `Never`): Defines the output type of
///   the operator, usually it's the same as the input type
/// - `#[rx_out_error(...)]` (optional, default: `Never`): Defines the output
///   error type of the operator, usually it's the same as the input error type
#[proc_macro_derive(
	RxOperator,
	attributes(rx_in, rx_in_error, rx_out, rx_out_error, _rx_core_traits_crate)
)]
pub fn operator_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	// skip for composables
	let primary_category_impl = impl_primary_category(&derive_input, primary_category_operator());
	let observable_output_impl = impl_observable_output(&derive_input);
	let observer_input_impl = impl_observer_input(&derive_input);

	(quote! {
		#primary_category_impl

		#observable_output_impl

		#observer_input_impl
	})
	.into()
}

use quote::quote;
use rx_core_macro_common::{
	derive_executor::impl_executor, derive_with_context_provider::impl_with_work_context_provider,
	derive_work::impl_with_work_input_output,
};
use syn::{DeriveInput, parse_macro_input};

/// # RxExecutor
///
/// Helper macro to implement a few traits required for an executor.
///
/// ## Traits Implemented
///
/// - `WorkContextProvider`: Using the type provided with `#[rx_context]`
/// - `WithWorkInputOutput`: Using the type provided with `#[rx_tick]`
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_context]`: The context type that is passed to the work when polled.
/// - `#[rx_tick]`: The tick type of work this scheduler can accept
/// - `#[rx_scheduler]`: The scheduler this executor exposes, must match the
///   context and work input/output types as this executor does
#[proc_macro_derive(
	RxExecutor,
	attributes(
		rx_context,
		rx_tick,
		rx_scheduler,
		scheduler_handle,
		_rx_core_common_crate
	)
)]
pub fn scheduler_executor_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let with_context_provider = impl_with_work_context_provider(&derive_input);
	let with_work_input_output = impl_with_work_input_output(&derive_input);
	let executor = impl_executor(&derive_input);

	(quote! {
		#with_context_provider

		#with_work_input_output

		#executor
	})
	.into()
}

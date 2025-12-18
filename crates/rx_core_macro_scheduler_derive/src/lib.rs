use quote::quote;
use rx_core_macro_common::{
	derive_task::impl_with_task_input_output,
	derive_with_context_provider::impl_with_context_provider,
};
use syn::{DeriveInput, parse_macro_input};

/// # RxScheduler
///
/// Helper macro to implement a few traits required for a scheduler.
///
/// ## Traits you still have to implement to get a scheduler
///
/// - `Scheduler`
///
/// ## Traits Implemented
///
/// - `ContextProvider`: Using the type provided with `#[rx_context]`
/// - `WithTaskInputOutput`: Using the type provided with `#[rx_tick]`
///
/// ## Attributes
///
/// > All attributes are prefixed with `rx_` for easy auto-complete access.
///
/// - `#[rx_context]`: The context type that is passed into tasks when polled.
/// - `#[rx_tick]`: The tick type of tasks this scheduler can accept
#[proc_macro_derive(RxScheduler, attributes(rx_context, rx_tick, _rx_core_traits_crate))]
pub fn scheduler_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);

	let with_context_provider = impl_with_context_provider(&derive_input);
	let with_task_input_output = impl_with_task_input_output(&derive_input);

	(quote! {

		#with_context_provider

		#with_task_input_output

	})
	.into()
}

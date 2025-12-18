use crate::Signal;

pub trait ObserverInput {
	type In: Signal;
	type InError: Signal;
}

/// # Observer
///
/// ## Signals & Channels
///
/// An Observer has three *signal channels*:
///
/// - `next`: carries **value** signals (`Self::In`)
/// - `error`: carries the **terminal error** signal (`Self::InError`)
/// - `complete`: carries the **terminal success** signal
///
/// Exactly one of `error` or `complete` may occur, and it may occur at
/// most once.
///
/// ## Example
///
/// ```rust
/// #[derive(RxObserver)]
/// #[rx_in(In)]
/// #[rx_in_error(InError)]
/// struct Print;
///
/// // Impletemented by the derive
/// // impl ObserverInput for Print {
/// //     type In = i32;
/// //     type InError = String;
/// // }
///
/// impl Observer for Print {
///     fn next(&mut self, next: Self::In) {
///         println!("next: {next}");
///     }
///     fn error(&mut self, error: Self::InError) {
///         eprintln!("error: {error}");
///     }
///     fn complete(&mut self) {
///         println!("complete");
///     }
/// }
/// ```
pub trait Observer: ObserverInput {
	fn next(&mut self, next: Self::In);
	fn error(&mut self, error: Self::InError);
	fn complete(&mut self);
}

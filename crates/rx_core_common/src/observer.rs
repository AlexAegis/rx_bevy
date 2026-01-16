use crate::Signal;

pub trait ObserverInput {
	type In: Signal;
	type InError: Signal;
}

/// # [RxObserver]
///
/// ## Signals & Channels
///
/// An RxObserver has three *signal channels*:
///
/// - `next`: carries **value** signals (`Self::In`)
/// - `error`: carries the **terminal error** signal (`Self::InError`)
/// - `complete`: carries the **terminal success** signal
///
/// Exactly one of `error` or `complete` may occur, and it may occur at
/// most once.
///
/// It's also possible that an observer observes no terminal signals if its
/// subscription was cancelled before it could. Some infinitely producing
/// observables (like `interval`) do not complete at all.
///
/// ## Example
///
/// ```rust
/// # use rx_core_macro_observer_derive::RxObserver;
/// # use rx_core_common::RxObserver;
///
/// #[derive(RxObserver)]
/// #[rx_in(i32)]
/// #[rx_in_error(String)]
/// struct Print;
///
/// // Implemented by the derive
/// // impl ObserverInput for Print {
/// //     type In = i32;
/// //     type InError = String;
/// // }
///
/// impl RxObserver for Print {
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
pub trait RxObserver: ObserverInput {
	/// Signals the next value.
	fn next(&mut self, next: Self::In);

	/// Signals an error of upstream, no more `next` or `complete` calls
	/// are expected after this!
	fn error(&mut self, error: Self::InError);

	/// Signals the completion of upstream, no more `next` or `error` calls
	/// are expected after this!
	fn complete(&mut self);
}

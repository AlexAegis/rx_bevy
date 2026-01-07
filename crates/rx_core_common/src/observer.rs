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
/// # use rx_core_macro_observer_derive::RxObserver;
/// # use rx_core_common::Observer;
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
/// TODO: Consider making all these fallible, NextError::Closed, NextError::Full, NextError::Blocked, CompleteError::Closed etc, including unsubscribe on SubscriptionLike
pub trait Observer: ObserverInput {
	/// Signals the next value.
	fn next(&mut self, next: Self::In);

	/// Signals an error of upstream, no more `next` or `complete` calls
	/// are expected after this!
	fn error(&mut self, error: Self::InError);

	/// Signals the completion of upstream, no more `next` or `error` calls
	/// are expected after this!
	fn complete(&mut self);
}

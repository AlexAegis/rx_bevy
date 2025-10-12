/// Built in core operators
///
/// Only those operators should be in this crate that are absolutely necessary
/// to be here because of an orphan rule.
mod option_operator;
mod option_subscriber;

pub use option_subscriber::*;

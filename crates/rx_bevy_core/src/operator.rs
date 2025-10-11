use crate::{ObservableOutput, ObserverInput, SubscriptionContext, Subscriber};

/// # [Operator]
///
/// An [Operator] is a mix between an [Observable] and an [Observer].
/// It defines its own inputs and outputs, and a
/// [Subscriber][Operator::Subscriber] that defines how those input signals
/// will produce output signals.
///
/// > What it does in essence, is wrapping a destination (a subscriber/observer)
/// > into another, defined by the operator, resulting in a new subscriber that
/// > can be used as a destination for something else.
/// >
/// > This lets you nest observers/subscribers into eachother, creating
/// > increasingly more complex [Observable][crate::Observable]s, by letting you
/// > define complex behavior, abstracted away in a
/// > [Subscriber][Operator::Subscriber].
///
/// Operators choose a single Context type for the whole subscriber chain
/// they participate in. Downstream and upstream must agree on this Context.
///
/// ## Pipes
///
/// To efficiently nest operators, the `Pipe` observable from `rx_bevy_pipe` can
/// be used. It provides a convenient set of functions for observables to chain
/// operators after observables, resulting in a new observable! A chaining api
/// is much more comfortable to use than nesting function calls, but under the
/// hood it's the same nested structure. Using 3 operators means you have an
/// Observable in a Pipe in a Pipe in a Pipe.
///
/// > This is provided in the `rx_bevy` crate through the `pipe` feature.
///
/// ## Composite Operator
///
/// Pipes only make sense when you wrap an observable to create a new, more
/// complex observable. But in some context you only want to define a new
/// operator, without having to write a new one. To this, you can use the
/// `rx_bevy_operator_composite` crate.
///
/// > This is provided in the `rx_bevy` crate through the `compose` feature.
///
/// Keep in mind that while creating new operators this way is very comfortable
/// and quick, when it comes to performance it may be better to write a new
/// operator.
///
pub trait Operator: ObserverInput + ObservableOutput {
	type Context: SubscriptionContext;

	type Subscriber<Destination>: 'static
		+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
		+ Send
		+ Sync
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;
}

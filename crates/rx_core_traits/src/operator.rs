use crate::{Observable, ObservableOutput, ObserverInput, Subscriber};

/// # [ComposableOperator]
///
/// Composable Operators are a subset of regular Operators. Unlike - for
/// example - the `retry` operator, that (as the name suggests) retries
/// subscription to the source, many other operators do not interact with their
/// source observable beyond just subscribing to them once.
///
/// They simply subscribe to the source once, and all they do is:
///
/// - Wrap the destination into a subscriber on subscribe
/// - And/Or Interact with the destination on subscribe
///   
///   > The `start_with` and `finalize` operators don't create anything new on
///   > subscribe, they only interact with the destination subscriber.
///
/// But they don't know anything about who the source observable is.
pub trait ComposableOperator: ObserverInput + ObservableOutput {
	type Subscriber<Destination>: 'static
		+ Subscriber<In = Self::In, InError = Self::InError>
		+ Send
		+ Sync
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;
}

pub trait Operator<'o>: ObserverInput + ObservableOutput {
	type OutObservable<InObservable>: 'o + Observable<Out = Self::Out, OutError = Self::OutError>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync;

	fn operate<InObservable>(self, source: InObservable) -> Self::OutObservable<InObservable>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync;
}

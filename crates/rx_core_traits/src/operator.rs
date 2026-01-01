use crate::{Observable, ObservableOutput, ObserverInput};

/// # [Operator]
///
/// Operators transform [Observable]s, giving them new behavior.
///
/// ## Composable Operators
///
/// Operators who just want to wrap the destination in a subscriber can also
/// implement [ComposableOperator][crate::ComposableOperator] instead.
/// Which allows the operator to be composable in addition.
pub trait Operator<'o>: ObserverInput + ObservableOutput {
	type OutObservable<InObservable>: 'o + Observable<Out = Self::Out, OutError = Self::OutError>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync;

	fn operate<InObservable>(self, source: InObservable) -> Self::OutObservable<InObservable>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync;
}

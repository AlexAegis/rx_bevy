use crate::{Observable, Subscriber};

/// A Subject is something that is an Observable and Observer (Subscriber) at
/// the same time. Signals pushed into it will be received by the subscriptions
/// made from it, broadcasting them.
pub trait SubjectLike: Observable<IsSubject = IsSubject> + Subscriber {}

impl<T> SubjectLike for T where T: Observable<IsSubject = IsSubject> + Subscriber {}

pub trait ObservableisSubjectQualifier: private::Seal + 'static {
	/// Boolean to indicate if this context is safe to create during a drop
	const IS_SUBJECT: bool;
}

#[derive(Debug)]
pub struct IsSubject;

impl private::Seal for IsSubject {}

impl ObservableisSubjectQualifier for IsSubject {
	const IS_SUBJECT: bool = true;
}

#[derive(Debug)]
pub struct NotSubject;

impl private::Seal for NotSubject {}

impl ObservableisSubjectQualifier for NotSubject {
	const IS_SUBJECT: bool = false;
}

/// 🦭
mod private {
	pub trait Seal {}
}

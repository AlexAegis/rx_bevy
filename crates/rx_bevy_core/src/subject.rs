use crate::{Observable, Subscriber};

/// A Subject is something that is an Observable and Observer (Subscriber) at
/// the same time. Signals pushed into it will be received by the subscriptions
/// made from it, broadcasting them.
pub trait SubjectLike: Clone + Observable + Subscriber {}

impl<T> SubjectLike for T where T: Clone + Observable + Subscriber {}

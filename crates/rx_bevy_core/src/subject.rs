use crate::{Observable, Subscriber};

pub trait SubjectLike: Clone + for<'c> Observable<'c> + Subscriber {}

impl<T> SubjectLike for T where T: Clone + for<'c> Observable<'c> + Subscriber {}

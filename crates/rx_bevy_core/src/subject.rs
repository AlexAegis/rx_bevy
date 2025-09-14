use crate::{Observable, Subscriber};

pub trait SubjectLike: Clone + Observable + Subscriber {}

impl<T> SubjectLike for T where T: Clone + Observable + Subscriber {}

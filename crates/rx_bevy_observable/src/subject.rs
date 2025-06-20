use crate::{Observable, Observer, SubscriptionLike};

pub trait SubjectLike: Clone + Observable + Observer + SubscriptionLike {}

impl<T> SubjectLike for T where T: SubjectLike {}

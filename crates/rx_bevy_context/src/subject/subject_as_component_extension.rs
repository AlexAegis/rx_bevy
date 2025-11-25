use rx_core_traits::{PrimaryCategorySubject, SubjectLike};

use crate::{RxBevyContext, SubjectComponent};

/// Convenience function to turn an observable into a component that can listen
/// to subscribe events.
pub trait SubjectAsComponentExtension:
	SubjectLike<Context = RxBevyContext, PrimaryCategory = PrimaryCategorySubject> + Send + Sync + Sized
where
	Self::In: Clone,
	Self::InError: Clone,
{
	fn into_component(self) -> SubjectComponent<Self>;
}

impl<Subject> SubjectAsComponentExtension for Subject
where
	Subject: SubjectLike<Context = RxBevyContext, PrimaryCategory = PrimaryCategorySubject>
		+ Send
		+ Sync
		+ Sized,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	fn into_component(self) -> SubjectComponent<Self> {
		SubjectComponent::new(self)
	}
}

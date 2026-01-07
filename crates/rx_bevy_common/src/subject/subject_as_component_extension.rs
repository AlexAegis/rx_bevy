use rx_core_common::{PrimaryCategorySubject, SubjectLike};

use crate::SubjectComponent;

/// Convenience function to turn an observable into a component that can listen
/// to subscribe events.
pub trait SubjectAsComponentExtension:
	SubjectLike<PrimaryCategory = PrimaryCategorySubject> + Send + Sync + Sized
where
	Self::In: Clone,
	Self::InError: Clone,
{
	fn into_component(self) -> SubjectComponent<Self>;
}

impl<Subject> SubjectAsComponentExtension for Subject
where
	Subject: SubjectLike<PrimaryCategory = PrimaryCategorySubject> + Send + Sync + Sized,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	fn into_component(self) -> SubjectComponent<Self> {
		SubjectComponent::new(self)
	}
}

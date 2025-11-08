use rx_core_traits::{PrimaryCategorySubject, SubjectLike};

use crate::{BevySubscriptionContextProvider, SubjectComponent};

/// Convenience function to turn an observable into a component that can listen
/// to subscribe events.
pub trait SubjectAsComponentExtension:
	SubjectLike<Context = BevySubscriptionContextProvider, PrimaryCategory = PrimaryCategorySubject>
	+ Send
	+ Sync
	+ Sized
{
	fn into_component(self) -> SubjectComponent<Self>;
}

impl<Subject> SubjectAsComponentExtension for Subject
where
	Subject: SubjectLike<
			Context = BevySubscriptionContextProvider,
			PrimaryCategory = PrimaryCategorySubject,
		> + Send
		+ Sync
		+ Sized,
{
	fn into_component(self) -> SubjectComponent<Self> {
		SubjectComponent::new(self)
	}
}

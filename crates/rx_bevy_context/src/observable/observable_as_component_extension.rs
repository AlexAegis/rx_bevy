use rx_core_traits::{Observable, PrimaryCategoryObservable};

use crate::{ObservableComponent, RxBevyContext};

/// Convenience function to turn an observable into a component that can listen
/// to subscribe events.
pub trait ObservableAsComponentExtension:
	Observable<Context = RxBevyContext, PrimaryCategory = PrimaryCategoryObservable>
	+ Send
	+ Sync
	+ Sized
{
	fn into_component(self) -> ObservableComponent<Self>;
}

impl<O> ObservableAsComponentExtension for O
where
	O: Observable<Context = RxBevyContext, PrimaryCategory = PrimaryCategoryObservable>
		+ Send
		+ Sync
		+ Sized,
{
	fn into_component(self) -> ObservableComponent<Self> {
		ObservableComponent::new(self)
	}
}

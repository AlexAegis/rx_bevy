use rx_bevy_observable::Observable;

use crate::MapOperator;

pub trait ObservableExtensionMap<Out>: Observable<Out = Out> + Sized {
	fn map<NextOut, F: Fn(Out) -> NextOut>(
		self,
		transform: F,
	) -> MapOperator<Self, Out, NextOut, F> {
		MapOperator::new_with_source(self, transform)
	}
}

impl<T, Out> ObservableExtensionMap<Out> for T where T: Observable<Out = Out> {}

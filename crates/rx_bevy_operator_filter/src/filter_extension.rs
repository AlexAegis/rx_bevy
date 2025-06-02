use rx_bevy_observable::Observable;
use rx_bevy_operator_pipe::Pipe;

use crate::FilterOperator;

pub trait ObservableExtensionFilter<T>: Observable<Out = T> + Sized {
	fn filter<F: for<'a> Fn(&'a T) -> bool>(
		self,
		filter: F,
	) -> Pipe<Self, FilterOperator<T, F, Self::Error>, Self::Error, Self::Error, T, T> {
		Pipe::new(self, FilterOperator::new(filter))
	}
}

impl<T, Out> ObservableExtensionFilter<Out> for T where T: Observable<Out = Out> {}

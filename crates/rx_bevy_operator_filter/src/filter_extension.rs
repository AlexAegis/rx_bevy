use rx_bevy_observable::Observable;
use rx_bevy_pipe_operator::Pipe;

use crate::FilterOperator;

pub trait ObservableExtensionFilter<Out>: Observable<Out = Out> + Sized {
	fn filter<Filter: Clone + for<'a> Fn(&'a Out) -> bool>(
		self,
		filter: Filter,
	) -> Pipe<Self, FilterOperator<Out, Self::OutError, Filter>> {
		Pipe::new(self, FilterOperator::new(filter))
	}
}

impl<T, Out> ObservableExtensionFilter<Out> for T where T: Observable<Out = Out> {}

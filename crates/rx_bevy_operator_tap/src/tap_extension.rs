use rx_bevy_observable::Observable;
use rx_bevy_operator::OperatorCallbackRef;

use crate::TapOperator;

pub trait ObservableExtensionTap<Out>: Observable<Out = Out> + Sized {
	fn tap<Callback: OperatorCallbackRef<Out, ()>>(
		self,
		callback: Callback,
	) -> TapOperator<Self, Out, Callback> {
		TapOperator::new_with_source(self, callback)
	}
}

impl<T, Out> ObservableExtensionTap<Out> for T where T: Observable<Out = Out> {}

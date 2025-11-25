use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::{
	AdsrTrigger,
	operator::{AdsrOperator, AdsrOperatorOptions},
};

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionAdsr: Observable<Out = AdsrTrigger> + Sized {
	fn adsr(
		self,
		options: AdsrOperatorOptions,
	) -> Pipe<Self, AdsrOperator<Self::OutError, Self::Context>> {
		Pipe::new(self, AdsrOperator::new(options))
	}
}

impl<Obs> ObservableExtensionAdsr for Obs where Obs: Observable<Out = AdsrTrigger> {}

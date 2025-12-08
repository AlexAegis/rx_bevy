use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::{
	AdsrTrigger,
	operator::{AdsrOperator, AdsrOperatorOptions},
};

pub trait ObservablePipeExtensionAdsr: Observable<Out = AdsrTrigger> + Sized {
	fn adsr(self, options: AdsrOperatorOptions) -> Pipe<Self, AdsrOperator<Self::OutError>> {
		Pipe::new(self, AdsrOperator::new(options))
	}
}

impl<O> ObservablePipeExtensionAdsr for O where O: Observable<Out = AdsrTrigger> {}

use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::SwitchMapOperator;

pub trait ObservablePipeExtensionSwitchMap: Observable + Sized {
	#[inline]
	fn switch_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + FnMut(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> <SwitchMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable> as Operator>::OutObservable<Self>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		SwitchMapOperator::new(mapper).operate(self)
	}
}

impl<O> ObservablePipeExtensionSwitchMap for O where O: Observable {}

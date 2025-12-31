use rx_core_traits::{Observable, Operator, Provider, SubjectLike};

use crate::operator::{ConnectableOptions, ShareOperator};

pub trait ObservablePipeExtensionShare: Observable + Sized {
	#[inline]
	fn share<ConnectorProvider>(
		self,
		options: ConnectableOptions<ConnectorProvider>,
	) -> <ShareOperator<ConnectorProvider> as Operator>::OutObservable<Self>
	where
		Self::Out: Clone,
		Self::OutError: Clone,
		ConnectorProvider: 'static + Provider,
		ConnectorProvider::Provided: SubjectLike<In = Self::Out, InError = Self::OutError> + Clone,
	{
		ShareOperator::new(options).operate(self)
	}
}

impl<O> ObservablePipeExtensionShare for O where O: Observable {}

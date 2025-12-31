use rx_core_traits::{Observable, Operator, Provider, SubjectLike};

use crate::operator::{ConnectableOptions, ShareOperator};

pub trait ObservablePipeExtensionShare<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn share<ConnectorProvider>(
		self,
		options: ConnectableOptions<ConnectorProvider>,
	) -> <ShareOperator<ConnectorProvider> as Operator<'o>>::OutObservable<Self>
	where
		Self::Out: Clone,
		Self::OutError: Clone,
		ConnectorProvider: 'static + Provider,
		ConnectorProvider::Provided: SubjectLike<In = Self::Out, InError = Self::OutError> + Clone,
	{
		ShareOperator::new(options).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionShare<'o> for O where O: 'o + Observable + Send + Sync {}

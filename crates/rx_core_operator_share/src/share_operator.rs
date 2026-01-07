use rx_core_common::{
	Observable, ObservableOutput, ObserverInput, Operator, Provider, SubjectLike,
};
use rx_core_macro_operator_derive::RxOperator;
use rx_core_observable_connectable::observable::ConnectableOptions;

use crate::observable::ShareObservable;

#[derive(RxOperator, Clone, Default)]
#[rx_in(<ConnectorProvider::Provided as ObserverInput>::In)]
#[rx_in_error(<ConnectorProvider::Provided as ObserverInput>::InError)]
#[rx_out(<ConnectorProvider::Provided as ObservableOutput>::Out)]
#[rx_out_error(<ConnectorProvider::Provided as ObservableOutput>::OutError)]
pub struct ShareOperator<ConnectorProvider>
where
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike + Clone,
{
	options: ConnectableOptions<ConnectorProvider>,
}

impl<ConnectorProvider> ShareOperator<ConnectorProvider>
where
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike + Clone,
{
	pub fn new(options: ConnectableOptions<ConnectorProvider>) -> Self {
		Self { options }
	}
}

impl<'o, ConnectorProvider> Operator<'o> for ShareOperator<ConnectorProvider>
where
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike + Clone,
	Self::In: Clone,
	Self::InError: Clone,
{
	type OutObservable<InObservable>
		= ShareObservable<InObservable, ConnectorProvider>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync;

	#[inline]
	fn operate<InObservable>(self, source: InObservable) -> Self::OutObservable<InObservable>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync,
	{
		ShareObservable::new(source, self.options)
	}
}

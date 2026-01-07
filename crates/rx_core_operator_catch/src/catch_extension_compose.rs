use rx_core_common::{ComposableOperator, Observable, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::CatchOperator;

pub trait OperatorComposeExtensionCatch: ComposableOperator + Sized {
	#[inline]
	fn catch<
		NextInnerObservable: Observable<Out = Self::Out> + Signal,
		OnError: 'static + FnOnce(Self::OutError) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		on_error: OnError,
	) -> CompositeOperator<
		Self,
		CatchOperator<Self::Out, Self::OutError, OnError, NextInnerObservable>,
	> {
		self.compose_with(CatchOperator::new(on_error))
	}
}

impl<Op> OperatorComposeExtensionCatch for Op where Op: ComposableOperator {}

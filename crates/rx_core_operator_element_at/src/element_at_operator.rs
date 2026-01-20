use core::marker::PhantomData;
use std::sync::Arc;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, PhantomInvariant, Provider, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::{ElementAtSubscriber, operator::ElementAtOperatorError};

/// # [ElementAtOperator]
///
/// Emit the value at the given index then complete.
///
/// If the element at the specified index does not exist, because it had
/// completed before reaching that index, the operator will either error
/// with [ElementAtOperatorError::IndexOutOfRange] or emit a default value
/// if one was provided.
#[derive_where(Debug, Clone)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(ElementAtOperatorError<InError>)]
pub struct ElementAtOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	index: usize,
	default_value: Option<Arc<dyn Provider<Provided = In> + Send + Sync>>,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> ElementAtOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(index: usize) -> Self {
		Self {
			index,
			default_value: None,
			_phantom_data: PhantomData,
		}
	}

	pub fn new_with_default<P>(index: usize, default_value: P) -> Self
	where
		P: 'static + Provider<Provided = In> + Send + Sync,
	{
		Self {
			index,
			default_value: Some(Arc::new(default_value)),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ComposableOperator for ElementAtOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= ElementAtSubscriber<In, InError, Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		ElementAtSubscriber::new(destination, self.index, self.default_value.clone())
	}
}

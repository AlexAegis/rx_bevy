use core::marker::PhantomData;
use std::sync::Arc;

use rx_core_common::{ComposableOperator, PhantomInvariant, Signal, Subscriber, Teardown};
use rx_core_macro_operator_derive::RxOperator;

#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct ErasedFinalizeOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	teardown_factory: Arc<dyn Fn() -> Box<dyn FnOnce() + Send + Sync> + Send + Sync>,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> ErasedFinalizeOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new<Callback>(teardown: Callback) -> Self
	where
		Callback: 'static + FnOnce() + Clone + Send + Sync,
	{
		Self {
			teardown_factory: Arc::new(move || {
				Box::new(teardown.clone()) as Box<dyn FnOnce() + Send + Sync>
			}),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ComposableOperator for ErasedFinalizeOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= Destination
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		mut destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		destination.add_teardown(Teardown::new_from_box((self.teardown_factory)()));
		destination
	}
}

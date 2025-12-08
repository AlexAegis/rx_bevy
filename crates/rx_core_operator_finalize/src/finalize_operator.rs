use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Signal, Subscriber, Teardown};

#[derive_where(Clone, Debug)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct FinalizeOperator<In, InError, Callback>
where
	In: Signal,
	InError: Signal,
	Callback: 'static + Clone + Into<Teardown> + Send + Sync,
{
	#[derive_where(skip(Debug))]
	callback: Callback,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Callback> FinalizeOperator<In, InError, Callback>
where
	In: Signal,
	InError: Signal,
	Callback: 'static + Clone + Into<Teardown> + Send + Sync,
{
	pub fn new(callback: Callback) -> Self {
		Self {
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback> Operator for FinalizeOperator<In, InError, Callback>
where
	In: Signal,
	InError: Signal,
	Callback: 'static + Clone + Into<Teardown> + Send + Sync,
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
		destination.add_teardown(self.callback.clone().into());
		destination
	}
}

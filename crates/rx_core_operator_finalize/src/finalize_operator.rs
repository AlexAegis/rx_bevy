use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{
	Operator, SignalBound, Subscriber, SubscriptionContext, TeardownCollectionExtension,
};

#[derive_where(Clone, Debug)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct FinalizeOperator<In, InError, Callback, Context = ()>
where
	In: SignalBound,
	InError: SignalBound,
	Callback: 'static + Clone + FnOnce(&mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	#[derive_where(skip(Debug))]
	callback: Callback,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Callback, Context> FinalizeOperator<In, InError, Callback, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Callback: 'static + Clone + FnOnce(&mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	pub fn new(callback: Callback) -> Self {
		Self {
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Callback, Context> Operator for FinalizeOperator<In, InError, Callback, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Callback: 'static + Clone + FnOnce(&mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= Destination
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		destination.add_fn(self.callback.clone(), context);
		destination
	}
}

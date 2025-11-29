use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Signal, Subscriber, SubscriptionContext};

use crate::FallbackWhenSilentSubscriber;

/// The [FallbackWhenSilentOperator] calls `into()` to map incoming values to the expected
/// out value provided `From` is implemented on the downstream type.
/// When both `In` and `Out`, and `InError` and `OutError` types are the same,
/// it's equivalent to the `identity` operator and is a noop.
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct FallbackWhenSilentOperator<In, InError, Fallback, Context = ()>
where
	In: Signal,
	InError: Signal,
	Fallback: 'static + Fn() -> In + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	fallback: Fallback,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, Fallback, Context> FallbackWhenSilentOperator<In, InError, Fallback, Context>
where
	In: Signal,
	InError: Signal,
	Fallback: 'static + Fn() -> In + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	pub fn new(fallback: Fallback) -> Self {
		Self {
			fallback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Fallback, Context> Operator
	for FallbackWhenSilentOperator<In, InError, Fallback, Context>
where
	In: Signal,
	InError: Signal,
	Fallback: 'static + Fn() -> In + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= FallbackWhenSilentSubscriber<In, InError, Fallback, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		FallbackWhenSilentSubscriber::new(destination, self.fallback.clone())
	}
}

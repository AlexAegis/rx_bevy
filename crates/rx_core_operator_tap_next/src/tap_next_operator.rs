use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, SignalBound, Subscriber, SubscriptionContext};

use crate::TapNextSubscriber;

#[derive(RxOperator, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct TapNextOperator<In, InError, OnNext, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	on_next: OnNext,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, OnNext, Context> TapNextOperator<In, InError, OnNext, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	pub fn new(on_next: OnNext) -> Self {
		Self {
			on_next,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext, Context> Operator for TapNextOperator<In, InError, OnNext, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= TapNextSubscriber<In, InError, OnNext, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

	#[inline]
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
		TapNextSubscriber::new(destination, self.on_next.clone())
	}
}

impl<In, InError, OnNext, Context> Clone for TapNextOperator<In, InError, OnNext, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		Self {
			on_next: self.on_next.clone(),
			_phantom_data: PhantomData,
		}
	}
}

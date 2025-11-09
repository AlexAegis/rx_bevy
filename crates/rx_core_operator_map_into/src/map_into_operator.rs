use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, SignalBound, Subscriber, SubscriptionContext};

use crate::MapIntoSubscriber;

/// The [MapIntoOperator] calls `into()` to map incoming values to the expected
/// out value provided `From` is implemented on the downstream type.
/// When both `In` and `Out`, and `InError` and `OutError` types are the same,
/// it's equivalent to the `identity` operator and is a noop.
#[derive_where(Debug, Clone, Default)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
#[rx_context(Context)]
pub struct MapIntoOperator<In, InError, Out, OutError, Context = ()>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
	Context: SubscriptionContext,
{
	pub _phantom_data: PhantomData<(In, InError, Out, OutError, Context)>,
}

impl<In, InError, Out, OutError, Context> Operator
	for MapIntoOperator<In, InError, Out, OutError, Context>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= MapIntoSubscriber<In, InError, Out, OutError, Destination>
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
		MapIntoSubscriber::new(destination)
	}
}

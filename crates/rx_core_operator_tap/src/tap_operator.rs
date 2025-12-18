use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Observer, Subscriber};

use crate::TapSubscriber;

#[derive_where(Debug, Clone)]
#[derive(RxOperator)]
#[rx_in(TapDestination::In)]
#[rx_in_error(TapDestination::InError)]
#[rx_out(TapDestination::In)]
#[rx_out_error(TapDestination::InError)]
pub struct TapOperator<TapDestination>
where
	TapDestination: 'static + Clone + Observer + Send + Sync,
	TapDestination::In: Clone,
	TapDestination::InError: Clone,
{
	#[derive_where(skip(Debug))]
	tap_destination: TapDestination,
}

impl<TapDestination> TapOperator<TapDestination>
where
	TapDestination: 'static + Clone + Observer + Send + Sync,
	TapDestination::In: Clone,
	TapDestination::InError: Clone,
{
	pub fn new(tap_destination: TapDestination) -> Self {
		Self { tap_destination }
	}
}

impl<TapDestination> ComposableOperator for TapOperator<TapDestination>
where
	TapDestination: 'static + Clone + Observer + Send + Sync,
	TapDestination::In: Clone,
	TapDestination::InError: Clone,
{
	type Subscriber<Destination>
		= TapSubscriber<TapDestination, Destination>
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
		TapSubscriber::new(destination, self.tap_destination.clone())
	}
}

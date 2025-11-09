use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observable, Observer, Subscriber, SubscriptionContext, WithSubscriptionContext,
};

use crate::{EitherOut2, EitherOutError2};

#[derive(RxSubscriber)]
#[rx_in(O1::Out)]
#[rx_in_error(O1::OutError)]
#[rx_context(<Destination as WithSubscriptionContext>::Context)]
#[rx_delegate_tickable_to_destination]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<In = EitherOut2<O1, O2>, InError = EitherOutError2<O1, O2>>,
{
	#[destination]
	destination: Destination,
	_phantom_data: PhantomData<(O1, O2)>,
}

impl<O1, O2, Destination> IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<In = EitherOut2<O1, O2>, InError = EitherOutError2<O1, O2>>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<O1, O2, Destination> Observer for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<In = EitherOut2<O1, O2>, InError = EitherOutError2<O1, O2>>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next(EitherOut2::O1(next), context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination
			.error(EitherOutError2::O1Error(error), context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.next(EitherOut2::CompleteO1, context);
		self.destination.complete(context);
	}
}

#[derive(RxSubscriber)]
#[rx_in(O2::Out)]
#[rx_in_error(O2::OutError)]
#[rx_context(<Destination as WithSubscriptionContext>::Context)]
#[rx_delegate_tickable_to_destination]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<In = EitherOut2<O1, O2>, InError = EitherOutError2<O1, O2>>,
{
	#[destination]
	destination: Destination,
	_phantom_data: PhantomData<(O1, O2)>,
}

impl<O1, O2, Destination> IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<In = EitherOut2<O1, O2>, InError = EitherOutError2<O1, O2>>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<O1, O2, Destination> Observer for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<In = EitherOut2<O1, O2>, InError = EitherOutError2<O1, O2>>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next(EitherOut2::O2(next), context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination
			.error(EitherOutError2::O2Error(error), context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.next(EitherOut2::CompleteO2, context);
		self.destination.complete(context);
	}
}

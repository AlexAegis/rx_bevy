use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown,
	Tick, Tickable, context::WithSubscriptionContext, prelude::SubscriptionContext,
};

use crate::{EitherOut2, EitherOutError2};

pub struct IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(O1, O2)>,
}

impl<O1, O2, Destination> IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<O1, O2, Destination> WithSubscriptionContext for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = <Destination as WithSubscriptionContext>::Context;
}

impl<O1, O2, Destination> Observer for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
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
		//self.destination.unsubscribe(context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.next(EitherOut2::CompleteO1, context);
		self.destination.complete(context);
		//self.destination.unsubscribe(context);
	}
}

impl<O1, O2, Destination> Tickable for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.tick(tick, context);
	}
}

impl<O1, O2, Destination> SubscriptionLike for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(
		&mut self,
		context: &mut <<Destination as WithSubscriptionContext>::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

impl<O1, O2, Destination> ObserverInput for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	type In = O1::Out;
	type InError = O1::OutError;
}

impl<O1, O2, Destination> ObservableOutput for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	type Out = EitherOut2<O1, O2>;
	type OutError = EitherOutError2<O1, O2>;
}

pub struct IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(O1, O2)>,
}

impl<O1, O2, Destination> IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<O1, O2, Destination> WithSubscriptionContext for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = <Destination as WithSubscriptionContext>::Context;
}

impl<O1, O2, Destination> Observer for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
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
		//self.destination.unsubscribe(context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.next(EitherOut2::CompleteO2, context);
		self.destination.complete(context);
		//self.destination.unsubscribe(context);
	}
}

impl<O1, O2, Destination> Tickable for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.tick(tick, context);
	}
}

impl<O1, O2, Destination> SubscriptionLike for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(
		&mut self,
		context: &mut <<Destination as WithSubscriptionContext>::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

impl<O1, O2, Destination> ObserverInput for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	type In = O2::Out;
	type InError = O2::OutError;
}

impl<O1, O2, Destination> ObservableOutput for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	type Out = EitherOut2<O1, O2>;
	type OutError = EitherOutError2<O1, O2>;
}

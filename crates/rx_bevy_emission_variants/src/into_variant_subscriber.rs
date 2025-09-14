use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

use crate::{EitherOut2, EitherOutError2};

pub struct IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(O1, O2)>,
}

impl<O1, O2, Destination> IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
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

impl<O1, O2, Destination> SignalContext for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = <Destination as SignalContext>::Context;
}

impl<O1, O2, Destination> Observer for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next(EitherOut2::O1(next), context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination
			.error(EitherOutError2::O1Error(error), context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.next(EitherOut2::CompleteO1, context);
		self.destination.complete(context);
	}

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<O1, O2, Destination> SubscriptionLike for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
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
	fn unsubscribe(&mut self, context: &mut <Destination as SignalContext>::Context) {
		self.destination.unsubscribe(context);
	}
}

impl<O1, O2, Destination> SubscriptionCollection for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		> + SubscriptionCollection,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<O1, O2, Destination> ObserverInput for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	type In = O1::Out;
	type InError = O1::OutError;
}

impl<O1, O2, Destination> ObservableOutput for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	type Out = EitherOut2<O1, O2>;
	type OutError = EitherOutError2<O1, O2>;
}

impl<O1, O2, Destination> Operation for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Destination = Destination;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}

pub struct IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(O1, O2)>,
}

impl<O1, O2, Destination> IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
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

impl<O1, O2, Destination> SignalContext for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = <Destination as SignalContext>::Context;
}

impl<O1, O2, Destination> Observer for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next(EitherOut2::O2(next), context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination
			.error(EitherOutError2::O2Error(error), context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.next(EitherOut2::CompleteO2, context);
		self.destination.complete(context);
	}

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<O1, O2, Destination> SubscriptionLike for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
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
	fn unsubscribe(&mut self, context: &mut <Destination as SignalContext>::Context) {
		self.destination.unsubscribe(context);
	}
}

impl<O1, O2, Destination> SubscriptionCollection for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		> + SubscriptionCollection,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<O1, O2, Destination> ObserverInput for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	type In = O2::Out;
	type InError = O2::OutError;
}

impl<O1, O2, Destination> ObservableOutput for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	type Out = EitherOut2<O1, O2>;
	type OutError = EitherOutError2<O1, O2>;
}

impl<O1, O2, Destination> Operation for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable<Context = Destination::Context>,
	O2: 'static + Observable<Context = Destination::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Destination = Destination;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}

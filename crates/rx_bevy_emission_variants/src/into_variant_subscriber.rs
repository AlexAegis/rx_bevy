use std::marker::PhantomData;

use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

use crate::{EitherOut2, EitherOutError2};

pub struct IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable,
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
	O1: 'static + Observable,
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

impl<O1, O2, Destination> Observer for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(EitherOut2::O1(next));
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(EitherOutError2::O1Error(error));
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.next(EitherOut2::CompleteO1);
		self.destination.complete();
	}

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(&mut self, tick: rx_bevy_observable::Tick) {
		self.destination.tick(tick);
	}
}

impl<O1, O2, Destination> SubscriptionLike for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
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
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: Box<dyn SubscriptionLike>) {
		self.destination.add(subscription);
	}
}

impl<O1, O2, Destination> ObserverInput for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable,
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
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	type Out = EitherOut2<O1, O2>;
	type OutError = EitherOutError2<O1, O2>;
}

impl<O1, O2, Destination> Operation for IntoVariant1of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
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
	O1: 'static + Observable,
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
	O1: 'static + Observable,
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

impl<O1, O2, Destination> Observer for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(EitherOut2::O2(next));
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(EitherOutError2::O2Error(error));
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.next(EitherOut2::CompleteO2);
		self.destination.complete();
	}

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(&mut self, tick: rx_bevy_observable::Tick) {
		self.destination.tick(tick);
	}
}

impl<O1, O2, Destination> SubscriptionLike for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
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
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: Box<dyn SubscriptionLike>) {
		self.destination.add(subscription);
	}
}

impl<O1, O2, Destination> ObserverInput for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable,
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
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: Subscriber,
{
	type Out = EitherOut2<O1, O2>;
	type OutError = EitherOutError2<O1, O2>;
}

impl<O1, O2, Destination> Operation for IntoVariant2of2Subscriber<O1, O2, Destination>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
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

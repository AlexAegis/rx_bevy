use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, ObservableOutput, Observer, ObserverInput, Operator};

pub struct MapOperator<In, Out, Mapper, Error>
where
	Mapper: Fn(In) -> Out,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<In, Out, Mapper, Error> Operator for MapOperator<In, Out, Mapper, Error>
where
	Mapper: Clone + Fn(In) -> Out,
{
	type Fw = MapForwarder<In, Out, Mapper, Error>;

	#[inline]
	fn create_instance(&self) -> Self::Fw {
		Self::Fw::new(self.mapper.clone())
	}
}

pub struct MapForwarder<In, Out, Mapper, Error>
where
	Mapper: Fn(In) -> Out,
{
	pub mapper: Mapper,
	pub index: u32,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<In, Out, Mapper, Error> MapForwarder<In, Out, Mapper, Error>
where
	Mapper: Fn(In) -> Out,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			index: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Out, F, Error> ObservableOutput for MapForwarder<In, Out, F, Error>
where
	F: Fn(In) -> Out,
{
	type Out = Out;
	type OutError = Error;
}

impl<In, Out, Mapper, Error> ObserverInput for MapForwarder<In, Out, Mapper, Error>
where
	Mapper: Fn(In) -> Out,
{
	type In = In;
	type InError = Error;
}

impl<In, Out, Mapper, Error> Forwarder for MapForwarder<In, Out, Mapper, Error>
where
	Mapper: Fn(In) -> Out,
{
	#[inline]
	fn next_forward<Destination: Observer<In = Out>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		let mapped = (self.mapper)(next);
		self.index += 1;
		destination.next(mapped);
	}

	#[inline]
	fn error_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		destination.error(error);
	}
}

impl<In, Out, Mapper, Error> MapOperator<In, Out, Mapper, Error>
where
	Mapper: Fn(In) -> Out,
{
	pub fn new(transform: Mapper) -> Self {
		Self {
			mapper: transform,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Out, Mapper, Error> Clone for MapOperator<In, Out, Mapper, Error>
where
	Mapper: Clone + Fn(In) -> Out,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}

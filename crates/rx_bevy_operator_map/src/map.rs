use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, ObservableOutput, Observer, ObserverInput, Operator};

pub struct MapOperator<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Out,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<Mapper, In, Out, Error> Operator for MapOperator<Mapper, In, Out, Error>
where
	Mapper: Clone + Fn(In) -> Out,
{
	type Fw = MapForwarder<Mapper, In, Out, Error>;

	#[inline]
	fn create_instance(&self) -> Self::Fw {
		Self::Fw::new(self.mapper.clone())
	}
}

pub struct MapForwarder<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Out,
{
	pub mapper: Mapper,
	pub index: u32,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<Mapper, In, Out, Error> MapForwarder<Mapper, In, Out, Error>
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

impl<Mapper, In, Out, Error> ObservableOutput for MapForwarder<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Out,
{
	type Out = Out;
	type OutError = Error;
}

impl<Mapper, In, Out, Error> ObserverInput for MapForwarder<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Out,
{
	type In = In;
	type InError = Error;
}

impl<Mapper, In, Out, Error> Forwarder for MapForwarder<Mapper, In, Out, Error>
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

impl<Mapper, In, Out, Error> MapOperator<Mapper, In, Out, Error>
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

impl<Mapper, In, Out, Error> Clone for MapOperator<Mapper, In, Out, Error>
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

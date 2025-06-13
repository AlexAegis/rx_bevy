use std::marker::PhantomData;

use rx_bevy_observable::{Forwarder, ObservableOutput, Observer, ObserverInput, Operator};

pub struct FilterMapOperator<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Option<Out>,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<Mapper, In, Out, Error> Operator for FilterMapOperator<Mapper, In, Out, Error>
where
	Mapper: Clone + Fn(In) -> Option<Out>,
{
	type Fw = FilterMapForwarder<Mapper, In, Out, Error>;

	#[inline]
	fn create_instance(&self) -> Self::Fw {
		Self::Fw::new(self.mapper.clone())
	}
}

pub struct FilterMapForwarder<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Option<Out>,
{
	pub mapper: Mapper,
	pub index: u32,
	pub _phantom_data: PhantomData<(In, Out, Error)>,
}

impl<Mapper, In, Out, Error> FilterMapForwarder<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Option<Out>,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			index: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<Mapper, In, Out, Error> ObservableOutput for FilterMapForwarder<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Option<Out>,
{
	type Out = Out;
	type OutError = Error;
}

impl<Mapper, In, Out, Error> ObserverInput for FilterMapForwarder<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Option<Out>,
{
	type In = In;
	type InError = Error;
}

impl<Mapper, In, Out, Error> Forwarder for FilterMapForwarder<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Option<Out>,
{
	#[inline]
	fn next_forward<Destination: Observer<In = Out>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		if let Some(mapped) = (self.mapper)(next) {
			self.index += 1;
			destination.next(mapped);
		}
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

impl<Mapper, In, Out, Error> FilterMapOperator<Mapper, In, Out, Error>
where
	Mapper: Fn(In) -> Option<Out>,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<Mapper, In, Out, Error> Clone for FilterMapOperator<Mapper, In, Out, Error>
where
	Mapper: Clone + Fn(In) -> Option<Out>,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}

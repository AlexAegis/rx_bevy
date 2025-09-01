use std::marker::PhantomData;

use rx_bevy_core::{Observable, ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::SwitchMapSubscriber;

pub struct SwitchMapOperator<In, InError, Switcher, InnerObservable> {
	pub switcher: Switcher,
	pub _phantom_data: PhantomData<(In, InError, InnerObservable)>,
}

impl<In, InError, Switcher, InnerObservable>
	SwitchMapOperator<In, InError, Switcher, InnerObservable>
{
	pub fn new(switcher: Switcher) -> Self {
		Self {
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, InnerObservable> Operator
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable,
{
	type Subscriber<Destination: Subscriber<In = Self::Out, InError = Self::OutError>> =
		SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>;

	fn operator_subscribe<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		SwitchMapSubscriber::new(destination, self.switcher.clone())
	}
}

impl<In, InError, Switcher, InnerObservable> ObserverInput
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Switcher, InnerObservable> ObservableOutput
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

impl<In, InError, Switcher, InnerObservable> Clone
	for SwitchMapOperator<In, InError, Switcher, InnerObservable>
where
	Switcher: Clone,
{
	fn clone(&self) -> Self {
		Self {
			switcher: self.switcher.clone(),
			_phantom_data: PhantomData,
		}
	}
}

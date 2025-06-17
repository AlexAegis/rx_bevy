use std::marker::PhantomData;

use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, ObserverInput, Operation, Operator, Subscriber,
	Subscription, SwitchSubscriber,
};

pub struct SwitchMapOperator<In, InError, Switcher, InnerObservable> {
	pub switcher: Switcher,
	pub _phantom_data: PhantomData<(In, InError, InnerObservable)>,
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

pub struct SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	destination: SwitchSubscriber<InnerObservable, Destination>,
	switcher: Switcher,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Switcher, InnerObservable, Destination>
	SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Clone + Fn(In) -> InnerObservable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub fn new(destination: Destination, switcher: Switcher) -> Self {
		Self {
			destination: SwitchSubscriber::new(destination),
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, InnerObservable, Destination> Observer
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: 'static + Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
{
	fn next(&mut self, next: Self::In) {
		self.destination.next((self.switcher)(next));
	}

	fn error(&mut self, error: Self::InError) {
		self.destination.error(error.into());
	}

	fn complete(&mut self) {
		self.destination.complete();
	}
}

impl<In, InError, Switcher, InnerObservable, Destination> ObserverInput
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Switcher, InnerObservable, Destination> ObservableOutput
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

impl<In, InError, Switcher, InnerObservable, Destination> Operation
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	type Destination = Destination;
}

impl<In, InError, Switcher, InnerObservable, Destination> Subscription
	for SwitchMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	InnerObservable: 'static + Observable,
	Switcher: Fn(In) -> InnerObservable,
	Destination: 'static + Observer<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}
}

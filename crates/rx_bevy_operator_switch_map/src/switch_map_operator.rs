use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, ObserverInput, Operator, ShareableSubscriber, SignalContext,
	Subscriber,
};

use crate::SwitchMapSubscriber;

pub struct SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = In, InError = InError, Context = InnerObservable::Context>,
	InnerObservable: 'static + Observable,
{
	pub switcher: Switcher,
	pub _phantom_data: PhantomData<(In, InError, Sharer, InnerObservable)>,
}

impl<In, InError, Switcher, Sharer, InnerObservable>
	SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = In, InError = InError, Context = InnerObservable::Context>,
	InnerObservable: 'static + Observable,
{
	pub fn new(switcher: Switcher) -> Self {
		Self {
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, Sharer, InnerObservable> Operator
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = In, InError = InError, Context = InnerObservable::Context>,
	InnerObservable: 'static + Observable,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		SwitchMapSubscriber<In, InError, Switcher, Sharer, InnerObservable, Destination>;

	fn operator_subscribe<
		Destination: Subscriber<In = Self::Out, InError = Self::OutError, Context = InnerObservable::Context>,
	>(
		&mut self,
		destination: Destination,
		_context: &mut Destination::Context,
	) -> Self::Subscriber<Destination> {
		SwitchMapSubscriber::new(destination, self.switcher.clone())
	}
}

impl<In, InError, Switcher, Sharer, InnerObservable> ObserverInput
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = In, InError = InError, Context = InnerObservable::Context>,
	InnerObservable: 'static + Observable,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Switcher, Sharer, InnerObservable> ObservableOutput
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = In, InError = InError, Context = InnerObservable::Context>,
	InnerObservable: 'static + Observable,
{
	type Out = InnerObservable::Out;
	type OutError = InnerObservable::OutError;
}

impl<In, InError, Switcher, Sharer, InnerObservable> SignalContext
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = In, InError = InError, Context = InnerObservable::Context>,
	InnerObservable: 'static + Observable,
{
	// TODO: Here maybe an Into context would make sense to downgrade contexts
	type Context = InnerObservable::Context;
}

impl<In, InError, Switcher, Sharer, InnerObservable> Clone
	for SwitchMapOperator<In, InError, Switcher, Sharer, InnerObservable>
where
	In: 'static,
	InError: 'static + Into<InnerObservable::OutError>,
	Switcher: 'static + Clone + Fn(In) -> InnerObservable,
	Sharer: 'static
		+ ShareableSubscriber<In = In, InError = InError, Context = InnerObservable::Context>,
	InnerObservable: 'static + Observable,
{
	fn clone(&self) -> Self {
		Self {
			switcher: self.switcher.clone(),
			_phantom_data: PhantomData,
		}
	}
}

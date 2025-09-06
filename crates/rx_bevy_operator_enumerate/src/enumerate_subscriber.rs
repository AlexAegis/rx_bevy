use std::marker::PhantomData;

#[cfg(feature = "channel_context")]
use rx_bevy_core::ChannelContext;
use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

pub struct EnumerateSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	destination: Destination,
	counter: usize,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> EnumerateSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			counter: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> Observer for EnumerateSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) {
		#[cfg(feature = "channel_context")]
		self.destination.next((next, self.counter), context);
		#[cfg(not(feature = "channel_context"))]
		self.destination.next((next, self.counter));

		// Increment after emission, so the first value could be 0
		#[cfg(feature = "saturating_add")]
		{
			self.counter = self.counter.saturating_add(1);
		}
		#[cfg(not(feature = "saturating_add"))]
		{
			self.counter += 1;
		}
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) {
		#[cfg(feature = "channel_context")]
		self.destination.error(error, context);
		#[cfg(not(feature = "channel_context"))]
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self, #[cfg(feature = "channel_context")] context: &mut ChannelContext) {
		#[cfg(feature = "channel_context")]
		self.destination.complete(context);
		#[cfg(not(feature = "channel_context"))]
		self.destination.complete();
	}

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(
		&mut self,
		tick: rx_bevy_core::Tick,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) {
		#[cfg(feature = "channel_context")]
		self.destination.tick(tick, context);
		#[cfg(not(feature = "channel_context"))]
		self.destination.tick(tick);
	}
}

impl<In, InError, Destination> SubscriptionLike for EnumerateSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
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
	fn add(&mut self, subscription: impl Into<Teardown>) {
		self.destination.add(subscription);
	}
}

impl<In, InError, Destination> ObserverInput for EnumerateSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Destination> ObservableOutput for EnumerateSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type Out = (In, usize);
	type OutError = InError;
}

impl<In, InError, Destination> Operation for EnumerateSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
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

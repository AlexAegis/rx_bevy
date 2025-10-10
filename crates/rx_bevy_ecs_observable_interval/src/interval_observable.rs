use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, SignalContext, Subscriber, SubscriptionHandle, WithContext,
};

use crate::{IntervalObservableOptions, IntervalSubscription};

pub struct IntervalObservable<Context>
where
	Context: SignalContext,
{
	options: IntervalObservableOptions,
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Context> IntervalObservable<Context>
where
	Context: SignalContext,
{
	pub fn new(options: IntervalObservableOptions) -> Self {
		Self {
			options,
			_phantom_data: PhantomData,
		}
	}
}

impl<Context> ObservableOutput for IntervalObservable<Context>
where
	Context: SignalContext,
{
	type Out = u32;
	type OutError = ();
}

impl<Context> WithContext for IntervalObservable<Context>
where
	Context: SignalContext,
{
	type Context = Context;
}

impl<Context> Observable for IntervalObservable<Context>
where
	Context: SignalContext,
{
	type Subscription = IntervalSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut Self::Context,
	) -> SubscriptionHandle<Self::Subscription>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		if self.options.start_on_subscribe {
			destination.next(0, context);
		}
		SubscriptionHandle::new(IntervalSubscription::new(destination, self.options.clone()))
	}
}

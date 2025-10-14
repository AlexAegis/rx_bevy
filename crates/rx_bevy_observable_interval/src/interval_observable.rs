use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, Subscriber,
	context::{SubscriptionContext, WithSubscriptionContext},
};

use crate::{IntervalObservableOptions, IntervalSubscription};

pub struct IntervalObservable<Context>
where
	Context: SubscriptionContext,
{
	options: IntervalObservableOptions,
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Context> IntervalObservable<Context>
where
	Context: SubscriptionContext,
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
	Context: SubscriptionContext,
{
	type Out = usize;
	type OutError = ();
}

impl<Context> WithSubscriptionContext for IntervalObservable<Context>
where
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Context> Observable for IntervalObservable<Context>
where
	Context: SubscriptionContext,
{
	type Subscription = IntervalSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) -> Self::Subscription
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		if self.options.start_on_subscribe {
			destination.next(0, context);
		}
		IntervalSubscription::new(destination, self.options.clone())
	}
}

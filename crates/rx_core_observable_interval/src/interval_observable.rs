use core::marker::PhantomData;

use rx_core_traits::{
	Observable, ObservableOutput, Observer, PrimaryCategoryObservable, SubscriptionContext,
	UpgradeableObserver, WithPrimaryCategory, WithSubscriptionContext,
};

use crate::{IntervalSubscription, observable::IntervalObservableOptions};

pub struct IntervalObservable<Context = ()>
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

impl<Context> WithPrimaryCategory for IntervalObservable<Context>
where
	Context: SubscriptionContext,
{
	type PrimaryCategory = PrimaryCategoryObservable;
}

impl<Context> Observable for IntervalObservable<Context>
where
	Context: SubscriptionContext,
{
	type Subscription = IntervalSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let mut destination = observer.upgrade();
		if self.options.start_on_subscribe {
			destination.next(0, context);
		}
		IntervalSubscription::new(destination, self.options.clone())
	}
}

use core::marker::PhantomData;

use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Never, Observable, Observer, Scheduler, Subscriber, SubscriptionContext, UpgradeableObserver,
};

use crate::{IntervalSubscription, observable::IntervalObservableOptions};

#[derive(RxObservable, Debug)]
#[rx_out(usize)]
#[rx_out_error(Never)]
#[rx_context(S::ContextProvider)]
pub struct IntervalObservable<S>
where
	S: Scheduler,
	S::ContextProvider: SubscriptionContext,
{
	options: IntervalObservableOptions<S>,
}

impl<S> IntervalObservable<S>
where
	S: Scheduler,
	S::ContextProvider: SubscriptionContext,
{
	pub fn new(options: IntervalObservableOptions<S>) -> Self {
		Self { options }
	}
}

impl<S> Observable for IntervalObservable<S>
where
	S: 'static + Scheduler + Send + Sync,
	S::ContextProvider: SubscriptionContext,
{
	type Subscription<Destination>
		= IntervalSubscription<S>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
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

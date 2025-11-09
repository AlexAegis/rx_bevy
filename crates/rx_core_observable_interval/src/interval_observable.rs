use core::marker::PhantomData;

use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Observable, Observer, SubscriptionContext, UpgradeableObserver};

use crate::{IntervalSubscription, observable::IntervalObservableOptions};

#[derive(RxObservable)]
#[rx_out(usize)]
#[rx_context(Context)]
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

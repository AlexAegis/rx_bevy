use core::marker::PhantomData;

use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;
use rx_core_traits::{
	Never, Observable, Observer, SignalBound, Subscriber, SubscriptionContext, UpgradeableObserver,
};

#[derive(RxObservable, Clone, Debug)]
#[rx_out(Never)]
#[rx_out_error(OutError)]
#[rx_context(Context)]
pub struct ThrowObservable<OutError, Context>
where
	OutError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	error: OutError,
	_phantom_data: PhantomData<Context>,
}

impl<OutError, Context> ThrowObservable<OutError, Context>
where
	OutError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	pub fn new(error: OutError) -> Self {
		Self {
			error,
			_phantom_data: PhantomData,
		}
	}
}

impl<OutError, Context> Observable for ThrowObservable<OutError, Context>
where
	OutError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Subscription<Destination>
		= InertSubscription<Context>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Context>,
	{
		let mut destination = observer.upgrade();
		destination.error(self.error.clone(), context);
		InertSubscription::new(destination, context)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use rx_core_testing::prelude::*;
	use rx_core_traits::DropSafeSubscriptionContext;

	#[test]
	fn should_emit_single_value() {
		let error = "error";
		let mut observable = ThrowObservable::new(error);
		let mock_observer = MockObserver::<_, _, DropSafeSubscriptionContext>::default();
		let mut mock_context = MockContext::default();

		let _s = observable.subscribe(mock_observer, &mut mock_context);

		assert_eq!(mock_context.all_observed_errors(), vec![error]);
	}
}

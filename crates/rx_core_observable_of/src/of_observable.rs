use core::marker::PhantomData;

use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;
use rx_core_traits::{
	Never, Observable, Observer, SignalBound, Subscriber, SubscriptionContext, UpgradeableObserver,
};

/// Emits a single value then immediately completes
#[derive(RxObservable, Clone, Debug)]
#[rx_out(Out)]
#[rx_out_error(Never)]
#[rx_context(Context)]
pub struct OfObservable<Out, Context = ()>
where
	Out: SignalBound + Clone,
	Context: SubscriptionContext,
{
	value: Out,
	_phantom_data: PhantomData<Context>,
}

impl<Out, Context> OfObservable<Out, Context>
where
	Out: SignalBound + Clone,
	Context: SubscriptionContext,
{
	pub fn new(value: Out) -> Self {
		Self {
			value,
			_phantom_data: PhantomData,
		}
	}
}

impl<Out, Context> Observable for OfObservable<Out, Context>
where
	Out: SignalBound + Clone,
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
		context: &mut Context::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let mut destination = observer.upgrade();
		destination.next(self.value.clone(), context);
		destination.complete(context);
		InertSubscription::new(destination, context)
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use rx_core_testing::{MockContext, MockObserver};
	use rx_core_traits::{DropSafeSubscriptionContext, SubscriptionLike};

	#[test]
	fn should_emit_single_value() {
		let value = 4;
		let mut observable = OfObservable::new(value);
		let mock_observer = MockObserver::<_, _, DropSafeSubscriptionContext>::default();
		let mut mock_context = MockContext::default();

		let mut subscription = observable.subscribe(mock_observer, &mut mock_context);
		subscription.unsubscribe(&mut mock_context);

		assert_eq!(mock_context.all_observed_values(), vec![value]);
	}
}

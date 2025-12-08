use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;
use rx_core_traits::{Never, Observable, Observer, Signal, Subscriber, UpgradeableObserver};

/// Emits a single value then immediately completes
#[derive(RxObservable, Clone, Debug)]
#[rx_out(Out)]
#[rx_out_error(Never)]
pub struct OfObservable<Out>
where
	Out: Signal + Clone,
{
	value: Out,
}

impl<Out> OfObservable<Out>
where
	Out: Signal + Clone,
{
	pub fn new(value: Out) -> Self {
		Self { value }
	}
}

impl<Out> Observable for OfObservable<Out>
where
	Out: Signal + Clone,
{
	type Subscription<Destination>
		= InertSubscription
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let mut destination = observer.upgrade();
		destination.next(self.value.clone());
		destination.complete();
		InertSubscription::new(destination)
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

		let mut subscription = observable.subscribe(mock_observer);
		subscription.unsubscribe();

		assert_eq!(mock_context.all_observed_values(), vec![value]);
	}
}

use crate::{ErasedSubscriber, Observable, Subscriber, SubscriptionData};

pub type BoxedSubscriber<In, InError> =
	Box<dyn 'static + Subscriber<In = In, InError = InError> + Send + Sync>;

pub trait ErasedSubscribeObservableExtension<Out, OutError> {
	fn erased_subscribe(&mut self, destination: BoxedSubscriber<Out, OutError>)
	-> SubscriptionData;
}

impl<O> ErasedSubscribeObservableExtension<O::Out, O::OutError> for O
where
	O: 'static + Observable + Send + Sync,
{
	fn erased_subscribe(
		&mut self,
		destination: Box<
			dyn 'static + Subscriber<In = O::Out, InError = O::OutError> + Send + Sync,
		>,
	) -> SubscriptionData {
		let subscription = self.subscribe(ErasedSubscriber::new(destination));
		SubscriptionData::new_with_teardown(subscription.into())
	}
}

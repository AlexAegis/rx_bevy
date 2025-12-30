use crate::{BoxedSubscriber, ErasedSubscriber, Observable, SubscriptionData};

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
		destination: BoxedSubscriber<O::Out, O::OutError>,
	) -> SubscriptionData {
		let subscription = self.subscribe(ErasedSubscriber::new(destination));
		SubscriptionData::new_with_teardown(subscription.into())
	}
}

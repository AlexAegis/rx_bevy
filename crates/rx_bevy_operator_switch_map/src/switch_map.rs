use std::marker::PhantomData;

use rx_bevy_higher_order_operator::{
	HigherOrderForwarder, HigherOrderObserver, HigherOrderOperator, HigherOrderSubscriber,
};
use rx_bevy_observable::{Observable, Observer, Subscription};

pub struct SwitchMapOperator<In, InnerObservable, Switcher> {
	pub switcher: Switcher,
	pub _phantom_data: PhantomData<(In, InnerObservable)>,
}
/*
impl<In, InnerObservable, Switcher> HigherOrderOperator
	for SwitchMapOperator<In, InnerObservable, Switcher>
where
	Switcher: Clone + Fn(In) -> InnerObservable,
	InnerObservable: Observable,
{
	type OutObservable = InnerObservable;
	type Subscriber = SwitchMapSubscriber<InnerObservable::Subscription>;

	fn higher_order_operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self::OutObservable as Observable>::Out,
				//Error = <Self::OutObservable as Observable>::Error,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> HigherOrderForwarder<Self::Subscriber, Destination> {
		HigherOrderForwarder::new(
			destination,
			SwitchMapSubscriber {
				inner_subscriber: None,
				switcher: self.switcher.clone(),
				_phantom_data: PhantomData,
			},
		)
	}
}
*/
pub struct SwitchMapSubscriber<In, InnerObservable, Switcher, InnerSubscriber>
where
	Switcher: Clone + Fn(In) -> InnerObservable,
	InnerSubscriber: Subscription,
{
	inner_subscriber: Option<InnerSubscriber>,
	switcher: Switcher,
	_phantom_data: PhantomData<In>,
}
/*
impl<In, InnerObservable, Switcher, InnerSubscriber> HigherOrderSubscriber
	for SwitchMapSubscriber<In, InnerObservable, Switcher, InnerSubscriber>
where
	Switcher: Clone + Fn(In) -> InnerObservable,
	InnerObservable: Observable,
	InnerSubscriber: Subscription,
{
	type In = In;

	fn subscribe_on_next(&mut self, next: Self::In, destination: Destination) {
		if let Some(mut inner_subscriber) = self.inner_subscriber {
			inner_subscriber.unsubscribe();
		}

		let inner_observable = (self.switcher)(next);
		let subscription = inner_observable.subscribe(observer);
		self.inner_subscriber = Some(subscription);
	}
}*/

impl<In, OutObservable, Switcher> SwitchMapOperator<In, OutObservable, Switcher> {
	pub fn new(switcher: Switcher) -> Self {
		Self {
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, OutObservable, Switcher> Clone for SwitchMapOperator<In, OutObservable, Switcher>
where
	Switcher: Clone,
{
	fn clone(&self) -> Self {
		Self {
			switcher: self.switcher.clone(),
			_phantom_data: PhantomData,
		}
	}
}

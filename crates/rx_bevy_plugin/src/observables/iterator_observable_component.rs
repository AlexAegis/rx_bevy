use crate::{CommandQuerySubscriber, ObservableComponent, SubscriptionComponent};
use bevy::prelude::*;
use rx_bevy::Observer;
use rx_bevy::prelude::*;
use std::fmt::Debug;

#[derive(Component, Clone, Debug)]
pub struct IteratorObservableComponent<Iterator>
where
	Iterator: Clone + IntoIterator,
{
	iterator: Iterator,
}

impl<Iterator> IteratorObservableComponent<Iterator>
where
	Iterator: Clone + IntoIterator,
{
	pub fn new(iterator: Iterator) -> Self {
		Self { iterator }
	}
}

impl<Iterator> ObservableComponent for IteratorObservableComponent<Iterator>
where
	Iterator: 'static + Clone + Send + Sync + IntoIterator + Debug,
	Iterator::Item: 'static + Send + Sync,
{
	fn component_subscribe(
		&mut self,
		mut subscriber: CommandQuerySubscriber<Self::Out, Self::OutError>,
	) -> SubscriptionComponent<Self::Out, Self::OutError> {
		for item in self.iterator.clone().into_iter() {
			if subscriber.is_closed() {
				break;
			}
			subscriber.next(item);
		}

		SubscriptionComponent::new(subscriber)
	}
}

impl<Iterator> ObservableOutput for IteratorObservableComponent<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: 'static,
{
	type Out = Iterator::Item;
	type OutError = ();
}

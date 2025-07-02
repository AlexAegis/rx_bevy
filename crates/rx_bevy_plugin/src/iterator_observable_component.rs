use bevy::prelude::*;
use rx_bevy::{
	IteratorObservable, Observable, ObservableOutput, Subscription, UpgradeableObserver,
};

#[derive(Component, Clone)]
pub struct IteratorObservableComponent<Iterator>
where
	Iterator: Clone + IntoIterator,
{
	iterator_observable: IteratorObservable<Iterator>,
}

impl<Iterator> IteratorObservableComponent<Iterator>
where
	Iterator: Clone + IntoIterator,
{
	pub fn new(iterator: Iterator) -> Self {
		Self {
			iterator_observable: IteratorObservable::new(iterator),
		}
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

impl<Iterator> Observable for IteratorObservableComponent<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: 'static,
{
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		self.iterator_observable.subscribe(destination)
	}
}

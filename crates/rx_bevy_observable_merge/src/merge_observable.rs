use std::{
	marker::PhantomData,
	sync::{Arc, RwLock},
};

use rx_bevy_observable::{
	DetachedSubscriber, Observable, ObservableOutput, SharedSubscriber, Subscriber, Subscription,
	SubscriptionLike, UpgradeableObserver,
};
use rx_bevy_operator_map_into::MapIntoSubscriber;
use rx_bevy_subject::MulticastDestination;
use slab::Slab;

use crate::{KeyedSubscriptionStore, ManyToOneKeyedSubscriber, SubscriptionStore};

/// Observable creator for [MergeObservable2]
pub fn merge<Out, OutError, O1, O2>(
	observable_bundle: ObservableBundle2<O1, O2>,
) -> MergeObservable2<Out, OutError, O1, O2>
where
	Out: 'static,
	OutError: 'static,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	MergeObservable2::new(observable_bundle)
}

/// This should be what the api expect, an into Observables, just like in Bevy's into Plugins thingy
pub trait Observables<Marker> {}

pub struct ObservableBundle2<O1, O2>(pub O1, pub O2)
where
	O1: Observable,
	O2: Observable;

pub struct SubscriptionBundle2(pub Subscription, pub Subscription);

pub struct MergeObservable2<Out, OutError, O1, O2>
where
	Out: 'static,
	OutError: 'static,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	observable_bundle: ObservableBundle2<O1, O2>,
	store: SubscriptionStore,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError, O1, O2> MergeObservable2<Out, OutError, O1, O2>
where
	Out: 'static,
	OutError: 'static,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	pub fn new(observable_bundle: ObservableBundle2<O1, O2>) -> Self {
		Self {
			observable_bundle,
			store: SubscriptionStore::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Out, OutError, O1, O2> ObservableOutput for MergeObservable2<Out, OutError, O1, O2>
where
	Out: 'static,
	OutError: 'static,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	type Out = Out;
	type OutError = OutError;
}

impl<Out, OutError, O1, O2> Observable for MergeObservable2<Out, OutError, O1, O2>
where
	Out: 'static,
	OutError: 'static,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		let mut subscription = Subscription::new_empty();

		let subscriber = SharedSubscriber::new(destination.upgrade());

		let (s1, _k1) = self.store.subscribe_with_store(
			&mut self.observable_bundle.0,
			MapIntoSubscriber::new(subscriber.clone()),
		);
		subscription.add(s1);

		let (s2, _k2) = self.store.subscribe_with_store(
			&mut self.observable_bundle.1,
			MapIntoSubscriber::new(subscriber.clone()),
		);
		subscription.add(s2);

		subscription
	}
}

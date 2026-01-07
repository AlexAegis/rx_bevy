use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use derive_where::derive_where;
use rx_core_common::prelude::*;
use rx_core_macro_observable_derive::RxObservable;

use crate::internal::{RetrySubscriber, SOURCE_STEAL};

#[derive_where(Clone)]
#[derive(RxObservable)]
#[rx_out(Source::Out)]
#[rx_out_error(Source::OutError)]
pub struct RetryObservable<'o, Source>
where
	Source: 'o + Observable,
{
	source: Arc<Mutex<Option<Source>>>,
	max_retries: usize,
	_phantom_data: PhantomData<&'o Source>,
}

impl<'o, Source> RetryObservable<'o, Source>
where
	Source: 'o + Observable,
{
	pub fn new(source: Source, max_retries: usize) -> Self {
		Self {
			source: Arc::new(Mutex::new(Some(source))),
			max_retries,
			_phantom_data: PhantomData,
		}
	}
}

impl<'o, Source> Observable for RetryObservable<'o, Source>
where
	Source: 'o + Observable + Send + Sync,
	'o: 'static,
{
	type Subscription<Destination>
		= SharedSubscription
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let mut shared_destination = SharedSubscriber::new(destination.upgrade());
		let mut outer_subscription = SharedSubscription::default();

		let mut immediate_retries = 0;

		let caught_error = Arc::new(Mutex::new(None));

		let last_subscription = Arc::new(Mutex::new(Option::<SharedSubscription>::None));

		while immediate_retries <= self.max_retries {
			caught_error.lock_ignore_poison().take();
			let mut stolen_source = self.source.lock_ignore_poison().take().expect(SOURCE_STEAL);

			let next_subscription = stolen_source.subscribe(RetrySubscriber::new(
				self.source.clone(),
				shared_destination.clone(),
				self.max_retries,
				immediate_retries,
				outer_subscription.clone(),
				last_subscription.clone(),
				caught_error.clone(),
			));

			if !next_subscription.is_closed() {
				last_subscription
					.lock_ignore_poison()
					.replace(SharedSubscription::new(next_subscription));
			}
			self.source.lock_ignore_poison().replace(stolen_source);

			immediate_retries += 1;

			if caught_error.lock_ignore_poison().is_some() {
				if let Some(mut last_subscription) = last_subscription.lock_ignore_poison().take() {
					last_subscription.unsubscribe();
				}
				continue;
			} else if shared_destination.is_closed() {
				break;
			} else {
				outer_subscription.add(last_subscription);
				break;
			}
		}

		if immediate_retries > self.max_retries {
			if let Some(error) = caught_error.lock_ignore_poison().take() {
				shared_destination.error(error);
			}
			shared_destination.unsubscribe();
			outer_subscription.unsubscribe();
		}

		outer_subscription.clone()
	}
}

use std::marker::PhantomData;

use bevy_ecs::resource::Resource;

use rx_bevy_context::BevySubscriptionContextProvider;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Observable, SignalBound, Subscriber, SubscriptionContext, UpgradeableObserver,
};

use crate::{ResourceSubscription, observable::ResourceObservableOptions};

#[derive(RxObservable)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
#[rx_context(BevySubscriptionContextProvider)]
pub struct ResourceObservable<R, Reader, Out, OutError>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Out, OutError> + Clone + Send + Sync,
	Out: SignalBound,
	OutError: SignalBound,
{
	reader: Reader,
	options: ResourceObservableOptions,
	_phantom_data: PhantomData<R>,
}

impl<R, Reader, Out, OutError> ResourceObservable<R, Reader, Out, OutError>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Out, OutError> + Clone + Send + Sync,
	Out: SignalBound,
	OutError: SignalBound,
{
	pub fn new(reader: Reader, options: ResourceObservableOptions) -> Self {
		Self {
			reader,
			options,
			_phantom_data: PhantomData,
		}
	}
}

impl<R, Reader, Out, OutError> Observable for ResourceObservable<R, Reader, Out, OutError>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Out, OutError> + Clone + Send + Sync,
	Out: SignalBound,
	OutError: SignalBound,
{
	type Subscription<Destination>
		= ResourceSubscription<R, Reader, Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		ResourceSubscription::new(
			self.reader.clone(),
			self.options.clone(),
			destination.upgrade(),
			context,
		)
	}
}

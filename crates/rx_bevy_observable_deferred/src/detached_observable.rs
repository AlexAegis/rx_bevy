use rx_bevy_core::{
	Observable, ObservableOutput, SignalContext, Subscriber, SubscriptionCollection,
};

pub fn detached_observable<'s, Source>(source: &'s mut Source) -> DetachedObservable<'s, Source>
where
	Source: Observable,
{
	DetachedObservable::new(source)
}

/// TODO: Reevaluate. This may be a dumb idea
pub struct DetachedObservable<'s, Source>
where
	Source: Observable,
{
	source: &'s mut Source,
}

impl<'s, Source> DetachedObservable<'s, Source>
where
	Source: Observable,
{
	pub fn new(source: &'s mut Source) -> Self {
		Self { source }
	}
}

impl<'s, Source> Observable for DetachedObservable<'s, Source>
where
	Source: Observable,
	Source::Subscription: 'static,
{
	type Subscription = Source::Subscription;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>
			+ SubscriptionCollection,
	{
		let subscription = self.source.subscribe(destination, context);

		let mut sub = Self::Subscription::default();
		sub.add_fn(
			move |_: &mut Destination::Context| {
				let _s = subscription;
			},
			context,
		);
		sub
	}
}

impl<'s, Source> ObservableOutput for DetachedObservable<'s, Source>
where
	Source: Observable,
{
	type Out = Source::Out;
	type OutError = Source::OutError;
}

use rx_bevy_core::{
	DropContext, DropSubscription, Observable, ObservableOutput, SignalContext, Teardown,
	UpgradeableObserver,
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
	<Source::Subscription as SignalContext>::Context: DropContext,
{
	type Subscription = DropSubscription<<Source::Subscription as SignalContext>::Context>;

	fn subscribe<
		Destination: 'static
			+ UpgradeableObserver<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Source::Subscription as SignalContext>::Context,
			>,
	>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> DropSubscription<Destination::Context>
	where
		Destination::Context: DropContext,
	{
		let subscription = self.source.subscribe(destination, context);

		DropSubscription::new(Teardown::new(Box::new(move || {
			let _s = subscription;
		})))
	}
}

impl<'s, Source> ObservableOutput for DetachedObservable<'s, Source>
where
	Source: Observable,
{
	type Out = Source::Out;
	type OutError = Source::OutError;
}

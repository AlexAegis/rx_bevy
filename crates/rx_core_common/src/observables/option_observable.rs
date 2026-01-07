use crate::{
	Observable, ObservableOutput, OptionSubscription, UpgradeableObserver, WithPrimaryCategory,
};

impl<O> ObservableOutput for Option<O>
where
	O: ObservableOutput,
{
	type Out = O::Out;
	type OutError = O::OutError;
}

impl<O> WithPrimaryCategory for Option<O>
where
	O: WithPrimaryCategory,
{
	type PrimaryCategory = O::PrimaryCategory;
}

impl<O> Observable for Option<O>
where
	O: Observable,
{
	type Subscription<Destination>
		= OptionSubscription<O::Subscription<<Destination as UpgradeableObserver>::Upgraded>>
	where
		Destination: 'static + crate::Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ crate::UpgradeableObserver<In = Self::Out, InError = Self::OutError>
			+ Send
			+ Sync,
	{
		OptionSubscription::new(self.as_mut().map(|obs| obs.subscribe(destination)))
	}
}

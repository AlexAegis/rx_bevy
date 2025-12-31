use crate::{Observer, ObserverInput, SubscriptionLike, TeardownCollection};

impl<O> ObserverInput for Option<O>
where
	O: ObserverInput,
{
	type In = O::In;
	type InError = O::InError;
}

impl<O> Observer for Option<O>
where
	O: Observer,
{
	fn next(&mut self, next: Self::In) {
		if let Some(destination) = self {
			destination.next(next);
		}
	}

	fn error(&mut self, error: Self::InError) {
		if let Some(destination) = self {
			destination.error(error);
		} else {
			panic!("Option Observer encountered an uncaught error!")
		}
	}

	fn complete(&mut self) {
		if let Some(destination) = self {
			destination.complete();
		}
	}
}

impl<O> TeardownCollection for Option<O>
where
	O: TeardownCollection,
{
	fn add_teardown(&mut self, teardown: crate::Teardown) {
		if let Some(destination) = self {
			destination.add_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}

impl<O> SubscriptionLike for Option<O>
where
	O: SubscriptionLike,
{
	fn is_closed(&self) -> bool {
		if let Some(destination) = self {
			destination.is_closed()
		} else {
			true
		}
	}

	fn unsubscribe(&mut self) {
		if let Some(destination) = self {
			destination.unsubscribe();
		}
	}
}

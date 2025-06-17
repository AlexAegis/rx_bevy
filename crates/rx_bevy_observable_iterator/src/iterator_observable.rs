use rx_bevy_observable::{Observable, ObservableOutput, Observer};

/// Emits a single value then immediately completes
#[derive(Clone)]
pub struct IteratorObservable<Iterator, Out>
where
	Iterator: Clone + IntoIterator<Item = Out>,
{
	iterator: Iterator,
}

impl<Iterator, Out> IteratorObservable<Iterator, Out>
where
	Iterator: Clone + IntoIterator<Item = Out>,
{
	pub fn new(iterator: Iterator) -> Self {
		Self { iterator }
	}
}

impl<Iterator, Out> ObservableOutput for IteratorObservable<Iterator, Out>
where
	Iterator: Clone + IntoIterator<Item = Out>,
	Out: 'static,
{
	type Out = Out;
	type OutError = ();
}

impl<Iterator, Out> Observable for IteratorObservable<Iterator, Out>
where
	Iterator: Clone + IntoIterator<Item = Out>,
	Out: 'static + Clone,
{
	type Subscription = ();

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: Observer<In = Out>>(
		&mut self,
		mut observer: Destination,
	) -> Self::Subscription {
		for item in self.iterator.clone().into_iter() {
			observer.next(item);
		}
		observer.complete();
	}
}

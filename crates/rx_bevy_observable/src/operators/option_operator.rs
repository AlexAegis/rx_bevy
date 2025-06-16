use crate::{ObservableOutput, Observer, ObserverInput, Operator, Subscriber, Subscription};

impl<T> Operator for Option<T>
where
	T: Operator,
{
	type Subscriber<D: Observer<In = Self::Out, InError = Self::OutError>> =
		OptionSubscriber<T::Subscriber<D>>;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		OptionSubscriber::new(
			self.as_mut()
				.map(|operator| operator.operator_subscribe(destination)),
		)
	}
}

impl<T> ObservableOutput for Option<T>
where
	T: Operator,
{
	type Out = T::Out;
	type OutError = T::OutError;
}

impl<T> ObserverInput for Option<T>
where
	T: Operator,
{
	type In = T::In;
	type InError = T::InError;
}

#[derive(Debug)]
pub struct OptionSubscriber<Sub>
where
	Sub: Subscriber,
{
	internal_subscriber: Option<Sub>,
}

impl<Sub> OptionSubscriber<Sub>
where
	Sub: Subscriber,
{
	pub fn new(internal_subscriber: Option<Sub>) -> Self {
		Self {
			internal_subscriber,
		}
	}
}

impl<Sub> ObservableOutput for OptionSubscriber<Sub>
where
	Sub: Subscriber,
{
	type Out = Sub::Out;
	type OutError = Sub::OutError;
}

impl<Sub> ObserverInput for OptionSubscriber<Sub>
where
	Sub: Subscriber,
{
	type In = Sub::In;
	type InError = Sub::InError;
}

impl<Sub> Observer for OptionSubscriber<Sub>
where
	Sub: Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if let Some(internal_subscriber) = &mut self.internal_subscriber {
			internal_subscriber.next(next);
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		if let Some(internal_subscriber) = &mut self.internal_subscriber {
			internal_subscriber.error(error);
		}
	}

	#[inline]
	fn complete(&mut self) {
		if let Some(internal_subscriber) = &mut self.internal_subscriber {
			internal_subscriber.complete();
		}
	}
}

impl<Sub> Subscriber for OptionSubscriber<Sub>
where
	Sub: Subscriber,
{
	type Destination = Sub::Destination;
}

impl<Sub> Subscription for OptionSubscriber<Sub>
where
	Sub: Subscriber,
{
	fn is_closed(&self) -> bool {
		self.internal_subscriber
			.as_ref()
			.map(|internal_sub| internal_sub.is_closed())
			.unwrap_or(true)
	}

	fn unsubscribe(&mut self) {
		self.internal_subscriber
			.as_mut()
			.map(|internal_sub| internal_sub.unsubscribe());
	}
}

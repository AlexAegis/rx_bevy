use crate::{ObservableOutput, ObserverInput, Operator, SubscriberForwarder};

impl<T> Operator for Option<T>
where
	T: Operator,
{
	type Sub<D> = OptionForwarder<T::Sub<D>>;

	fn create_instance<D>(&self) -> Self::Sub<D> {
		OptionForwarder::new(self.as_ref().map(|operator| operator.create_instance()))
	}
}

impl<T> ObservableOutput for Option<T>
where
	T: Operator,
{
	type Out = <T as ObservableOutput>::Out;
	type OutError = <T as ObservableOutput>::OutError;
}

impl<T> ObserverInput for Option<T>
where
	T: Operator,
{
	type In = <T as ObserverInput>::In;
	type InError = <T as ObserverInput>::InError;
}

#[derive(Debug)]
pub struct OptionForwarder<Fw>
where
	Fw: SubscriberForwarder,
{
	optional_internal_forwarder: Option<Fw>,
}

impl<Fw> OptionForwarder<Fw>
where
	Fw: SubscriberForwarder,
{
	pub fn new(optional_internal_forwarder: Option<Fw>) -> Self {
		Self {
			optional_internal_forwarder,
		}
	}
}

impl<Fw> Default for OptionForwarder<Fw>
where
	Fw: SubscriberForwarder,
{
	fn default() -> Self {
		Self {
			optional_internal_forwarder: None,
		}
	}
}

impl<Fw> ObservableOutput for OptionForwarder<Fw>
where
	Fw: SubscriberForwarder,
{
	type Out = <Fw as ObservableOutput>::Out;
	type OutError = <Fw as ObservableOutput>::OutError;
}

impl<Fw> ObserverInput for OptionForwarder<Fw>
where
	Fw: SubscriberForwarder,
{
	type In = <Fw as ObserverInput>::In;
	type InError = <Fw as ObserverInput>::InError;
}

impl<Fw> SubscriberForwarder for OptionForwarder<Fw>
where
	Fw: SubscriberForwarder,
{
	type Destination = Fw::Destination;

	#[inline]
	fn next_forward(&mut self, next: Self::In, destination: &mut Self::Destination) {
		if let Some(internal_forwarder) = &mut self.optional_internal_forwarder {
			internal_forwarder.next_forward(next, destination);
		}
	}

	#[inline]
	fn error_forward(&mut self, error: Self::InError, destination: &mut Self::Destination) {
		if let Some(internal_forwarder) = &mut self.optional_internal_forwarder {
			internal_forwarder.error_forward(error, destination);
		}
	}

	#[inline]
	fn complete_forward(&mut self, destination: &mut Self::Destination) {
		if let Some(internal_forwarder) = &mut self.optional_internal_forwarder {
			internal_forwarder.complete_forward(destination);
		}
	}
}

impl<Fw> Clone for OptionForwarder<Fw>
where
	Fw: Clone + SubscriberForwarder,
{
	fn clone(&self) -> Self {
		Self {
			optional_internal_forwarder: self.optional_internal_forwarder.clone(),
		}
	}
}

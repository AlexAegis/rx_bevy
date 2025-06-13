use crate::{Forwarder, ObservableOutput, Observer, ObserverInput, Operator, Subscriber};

impl<T> Operator for Option<T>
where
	T: Operator,
{
	type Fw = OptionForwarder<T::Fw>;

	fn create_instance(&self) -> Self::Fw {
		OptionForwarder::new(self.as_ref().map(|operator| operator.create_instance()))
	}

	fn operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self::Fw as ObservableOutput>::Out,
				InError = <Self::Fw as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscriber<Self::Fw, Destination> {
		Subscriber::new(destination, self.create_instance())
	}
}

#[derive(Debug)]
pub struct OptionForwarder<Fw>
where
	Fw: Forwarder,
{
	optional_internal_forwarder: Option<Fw>,
}

impl<Fw> OptionForwarder<Fw>
where
	Fw: Forwarder,
{
	pub fn new(optional_internal_forwarder: Option<Fw>) -> Self {
		Self {
			optional_internal_forwarder,
		}
	}
}

impl<Fw> Default for OptionForwarder<Fw>
where
	Fw: Forwarder,
{
	fn default() -> Self {
		Self {
			optional_internal_forwarder: None,
		}
	}
}

impl<Fw> ObservableOutput for OptionForwarder<Fw>
where
	Fw: Forwarder,
{
	type Out = <Fw as ObservableOutput>::Out;
	type OutError = <Fw as ObservableOutput>::OutError;
}

impl<Fw> ObserverInput for OptionForwarder<Fw>
where
	Fw: Forwarder,
{
	type In = <Fw as ObserverInput>::In;
	type InError = <Fw as ObserverInput>::InError;
}

impl<Fw> Forwarder for OptionForwarder<Fw>
where
	Fw: Forwarder,
{
	#[inline]
	fn next_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		if let Some(internal_forwarder) = &mut self.optional_internal_forwarder {
			internal_forwarder.next_forward(next, destination);
		}
	}

	#[inline]
	fn error_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		error: Self::InError,
		destination: &mut Destination,
	) {
		if let Some(internal_forwarder) = &mut self.optional_internal_forwarder {
			internal_forwarder.error_forward(error, destination);
		}
	}

	#[inline]
	fn complete_forward<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: &mut Destination,
	) {
		if let Some(internal_forwarder) = &mut self.optional_internal_forwarder {
			internal_forwarder.complete_forward(destination);
		}
	}
}

impl<Fw> Clone for OptionForwarder<Fw>
where
	Fw: Clone + Forwarder,
{
	fn clone(&self) -> Self {
		Self {
			optional_internal_forwarder: self.optional_internal_forwarder.clone(),
		}
	}
}

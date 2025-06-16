use crate::{Observer, ObserverInput, SubscriberForwarder};

/// Combines an operator instance and a destination
/// Useful for combining operators, as a short lived struct
pub struct IntermediateObserver<'a, Fw, Dest>
where
	Fw: SubscriberForwarder<Destination = Dest>,
	Dest: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	pub operator_instance: &'a mut Fw,
	pub destination: &'a mut Dest,
}

impl<'a, Fw, Dest> IntermediateObserver<'a, Fw, Dest>
where
	Fw: SubscriberForwarder<Destination = Dest>,
	Dest: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	pub fn new(operator_instance: &'a mut Fw, destination: &'a mut Dest) -> Self {
		Self {
			operator_instance,
			destination,
		}
	}
}

impl<'a, Fw, Dest> ObserverInput for IntermediateObserver<'a, Fw, Dest>
where
	Fw: SubscriberForwarder<Destination = Dest>,
	Dest: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	type In = Fw::In;
	type InError = Fw::InError;
}

impl<'a, Fw, Dest> Observer for IntermediateObserver<'a, Fw, Dest>
where
	Fw: SubscriberForwarder<Destination = Dest>,
	Dest: Observer<In = Fw::Out, InError = Fw::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.operator_instance.next_forward(next, self.destination);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.operator_instance
			.error_forward(error, self.destination);
	}

	#[inline]
	fn complete(&mut self) {
		self.operator_instance.complete_forward(self.destination);
	}
}

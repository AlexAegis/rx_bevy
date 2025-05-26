use rx_bevy_observable::Observer;

/// The internal implementation detail of an operator, defines how a value
/// should be pushed into a [Destination]
///
/// Internally, an [OperatorInstance] is the part of a [OperatorInstanceForwardObserver]
pub trait OperatorInstance {
	type In;
	type Out;

	fn push_forward<Destination: Observer<In = Self::Out>>(
		&mut self,
		value: Self::In,
		destination: &mut Destination,
	);
}

/// An observer that contains a concrete [Destination] and an [OperatorInstance]
/// implementation
/// It's used to connect the internal forwarders of operators to an observer
/// It's mostly only used as an internal detail of operators.
pub struct OperatorInstanceForwardObserver<
	In,
	Out,
	Instance: OperatorInstance<In = In>,
	Destination: Observer<In = Out>,
> {
	pub instance: Instance,
	pub destination: Destination,
}

impl<In, Out, Instance, Destination> OperatorInstanceForwardObserver<In, Out, Instance, Destination>
where
	Instance: OperatorInstance<In = In>,
	Destination: Observer<In = Out>,
{
	pub fn new(instance: Instance, destination: Destination) -> Self {
		Self {
			instance,
			destination,
		}
	}
}

impl<In, Out, F, Destination> Observer for OperatorInstanceForwardObserver<In, Out, F, Destination>
where
	F: OperatorInstance<In = In, Out = Out>,
	Destination: Observer<In = Out>,
{
	type In = In;

	fn on_push(&mut self, value: Self::In) {
		self.instance.push_forward(value, &mut self.destination);
	}
}

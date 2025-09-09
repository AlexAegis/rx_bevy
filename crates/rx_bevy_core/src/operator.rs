use std::ops::{Deref, DerefMut};

use crate::{ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber};

/// # [Operator]
///
/// An [Operator] defines its own inputs and output, and a [OperationSubscriber]
/// that defines how those input signals will produce output signals.
pub trait Operator: ObserverInput + ObservableOutput + Clone {
	// TODO: Should be into destination context so the context can be downgraded along the operators
	type Subscriber<Destination>: OperationSubscriber<
			Destination = Destination,
			In = Self::In,
			InError = Self::InError,
			Context = Destination::Context,
		>
	where
		Destination: Subscriber<In = Self::Out, InError = Self::OutError>;

	fn operator_subscribe<'c, Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Subscriber<Destination> as SignalContext>::Context,
	) -> Self::Subscriber<Destination>;
}

/// An [OperationSubscriber] is a more specialized version of a [Subscriber]
/// used by [Operators]. It's a [Subscriber] that is aware of its Destination
/// because it has constrains on its own outputs.
pub trait OperationSubscriber: Subscriber + Operation {}

impl<T> OperationSubscriber for T where T: Subscriber + Operation {}

/// An operation is something that does something to its [`Self::Destination`]
pub trait Operation {
	type Destination: Observer;

	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination);

	fn write_destination<F>(&mut self, writer: F)
	where
		F: FnMut(&mut Self::Destination);
}

impl<T, Target> Operation for T
where
	Target: 'static + Operation,
	T: Deref<Target = Target> + DerefMut<Target = Target>,
{
	type Destination = Target::Destination;

	/// Let's you check the shared observer for the duration of the callback
	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		self.deref().read_destination(reader);
	}

	/// Let's you check the shared observer for the duration of the callback
	#[inline]
	fn write_destination<F>(&mut self, writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		self.deref_mut().write_destination(writer);
	}
}

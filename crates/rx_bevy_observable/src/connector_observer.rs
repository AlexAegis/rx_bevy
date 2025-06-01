use crate::Observer;

pub trait ObserverConnector {
	type In;
	type Out;

	fn push_forward<Destination: Observer<In = Self::Out>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	);
}

pub trait DynObserverConnector {
	type In;
	type Out;

	fn push_forward(&mut self, next: Self::In, destination: &mut dyn Observer<In = Self::Out>);
}

impl<T> ObserverConnector for T
where
	T: DynObserverConnector,
{
	type In = T::In;
	type Out = T::Out;

	fn push_forward<Destination: Observer<In = Self::Out>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		DynObserverConnector::push_forward(self, next, destination);
	}
}

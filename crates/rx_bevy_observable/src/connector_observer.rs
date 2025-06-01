use crate::Observer;

pub trait ConnectorObserver {
	type In;
	type Out;

	fn push_forward<Destination: Observer<Self::Out>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	);
}

pub trait DynConnectorObserver {
	type In;
	type Out;

	fn push_forward(&mut self, next: Self::In, destination: &mut dyn Observer<Self::Out>);
}

impl<T> ConnectorObserver for T
where
	T: DynConnectorObserver,
{
	type In = T::In;
	type Out = T::Out;

	fn push_forward<Destination: Observer<Self::Out>>(
		&mut self,
		next: Self::In,
		destination: &mut Destination,
	) {
		DynConnectorObserver::push_forward(self, next, destination);
	}
}

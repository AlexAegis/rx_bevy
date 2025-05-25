pub trait Observer {
	type In;

	fn on_push(&mut self, value: Self::In);
}

pub trait Forwarder {
	type In;
	type Out;

	fn push_forward<Destination: Observer<In = Self::Out>>(
		&mut self,
		value: Self::In,
		destination: &mut Destination,
	);
}

use crate::observers::Observer;

pub trait Observable {
	type Out;

	fn subscribe<Destination: Observer<In = Self::Out>>(self, observer: Destination);
}

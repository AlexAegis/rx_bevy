use crate::Observer;

pub trait Observable {
	type Out;

	/// TODO: This shouldn't be consuming on the outer layer
	fn subscribe<Destination: Observer<In = Self::Out>>(self, observer: Destination);
}

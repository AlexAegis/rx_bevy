use crate::observers::Observer;

pub trait Observable<Destination>
where
	Destination: Observer<In = Self::Out>,
{
	type Out;

	fn internal_subscribe(self, observer: Destination);
}

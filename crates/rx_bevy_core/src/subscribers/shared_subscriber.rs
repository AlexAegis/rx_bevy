use crate::Subscriber;

pub trait SharedSubscriber: Subscriber + Clone
where
	Self: Sized,
{
	fn share<Destination>(destination: Destination) -> Self
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;
}

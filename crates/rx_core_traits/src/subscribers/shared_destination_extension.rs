use crate::Subscriber;

pub trait SharedDestination<Destination>
where
	Destination: 'static + ?Sized + Subscriber + Send + Sync,
{
	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Destination);

	fn access_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut Destination);
}

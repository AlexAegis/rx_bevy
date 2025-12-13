use std::sync::{Arc, Mutex, MutexGuard};

pub trait LockWithPoisonBehavior<T>
where
	T: 'static,
{
	type Guard<'g>
	where
		Self: 'g;

	fn lock_with_poison_behavior<F: FnOnce(&mut Self::Guard<'_>)>(
		&self,
		if_poisoned: F,
	) -> Self::Guard<'_>;

	fn lock_ignore_poison(&self) -> Self::Guard<'_>;
}

impl<T> LockWithPoisonBehavior<T> for Arc<Mutex<T>>
where
	T: 'static,
{
	type Guard<'g>
		= MutexGuard<'g, T>
	where
		Self: 'g;

	#[inline]
	fn lock_with_poison_behavior<F: FnOnce(&mut Self::Guard<'_>)>(
		&self,
		if_poisoned: F,
	) -> Self::Guard<'_> {
		self.lock().unwrap_or_else(|poison_error| {
			let mut inner = poison_error.into_inner();
			(if_poisoned)(&mut inner);
			inner
		})
	}

	#[inline]
	fn lock_ignore_poison(&self) -> Self::Guard<'_> {
		self.lock()
			.unwrap_or_else(|poison_error| poison_error.into_inner())
	}
}

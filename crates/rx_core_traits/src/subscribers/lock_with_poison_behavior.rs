use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait LockWithPoisonBehavior<T>
where
	T: 'static + ?Sized,
{
	type Guard<'g>
	where
		Self: 'g;

	fn lock_with_poison_behavior<F: FnOnce(&mut Self::Guard<'_>)>(
		&self,
		if_poisoned: F,
	) -> Self::Guard<'_>;

	fn lock_ignore_poison(&self) -> Self::Guard<'_>;

	fn lock_clear_poison(&self) -> Self::Guard<'_>;
}

impl<T> LockWithPoisonBehavior<T> for Arc<Mutex<T>>
where
	T: 'static + ?Sized,
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

	#[inline]
	fn lock_clear_poison(&self) -> Self::Guard<'_> {
		self.lock().unwrap_or_else(|poison_error| {
			self.clear_poison();
			poison_error.into_inner()
		})
	}
}

pub trait WriteLockWithPoisonBehavior<T>
where
	T: 'static + ?Sized,
{
	type Guard<'g>
	where
		Self: 'g;

	fn write_lock_with_poison_behavior<F: FnOnce(&mut Self::Guard<'_>)>(
		&self,
		if_poisoned: F,
	) -> Self::Guard<'_>;

	fn write_lock_ignore_poison(&self) -> Self::Guard<'_>;

	fn write_lock_clear_poison(&self) -> Self::Guard<'_>;
}

impl<T> WriteLockWithPoisonBehavior<T> for Arc<RwLock<T>>
where
	T: 'static + ?Sized,
{
	type Guard<'g>
		= RwLockWriteGuard<'g, T>
	where
		Self: 'g;

	#[inline]
	fn write_lock_with_poison_behavior<F: FnOnce(&mut Self::Guard<'_>)>(
		&self,
		if_poisoned: F,
	) -> Self::Guard<'_> {
		self.write().unwrap_or_else(|poison_error| {
			let mut inner = poison_error.into_inner();
			(if_poisoned)(&mut inner);
			inner
		})
	}

	#[inline]
	fn write_lock_ignore_poison(&self) -> Self::Guard<'_> {
		self.write()
			.unwrap_or_else(|poison_error| poison_error.into_inner())
	}

	#[inline]
	fn write_lock_clear_poison(&self) -> Self::Guard<'_> {
		self.write().unwrap_or_else(|poison_error| {
			self.clear_poison();
			poison_error.into_inner()
		})
	}
}

pub trait ReadLockWithPoisonBehavior<T>
where
	T: 'static + ?Sized,
{
	type Guard<'g>
	where
		Self: 'g;

	fn read_lock_with_poison_behavior<F: FnOnce(&mut Self::Guard<'_>)>(
		&self,
		if_poisoned: F,
	) -> Self::Guard<'_>;

	fn read_lock_ignore_poison(&self) -> Self::Guard<'_>;

	fn read_lock_clear_poison(&self) -> Self::Guard<'_>;
}

impl<T> ReadLockWithPoisonBehavior<T> for Arc<RwLock<T>>
where
	T: 'static + ?Sized,
{
	type Guard<'g>
		= RwLockReadGuard<'g, T>
	where
		Self: 'g;

	#[inline]
	fn read_lock_with_poison_behavior<F: FnOnce(&mut Self::Guard<'_>)>(
		&self,
		if_poisoned: F,
	) -> Self::Guard<'_> {
		self.read().unwrap_or_else(|poison_error| {
			let mut inner = poison_error.into_inner();
			(if_poisoned)(&mut inner);
			inner
		})
	}

	#[inline]
	fn read_lock_ignore_poison(&self) -> Self::Guard<'_> {
		self.read()
			.unwrap_or_else(|poison_error| poison_error.into_inner())
	}

	#[inline]
	fn read_lock_clear_poison(&self) -> Self::Guard<'_> {
		self.read().unwrap_or_else(|poison_error| {
			self.clear_poison();
			poison_error.into_inner()
		})
	}
}

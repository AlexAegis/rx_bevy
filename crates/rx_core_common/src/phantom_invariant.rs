use std::marker::PhantomData;

pub type PhantomInvariant<T> = PhantomData<fn(T) -> T>;

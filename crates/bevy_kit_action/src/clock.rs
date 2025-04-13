use bevy::reflect::{FromReflect, GetTypeRegistration, Typed};

pub trait Clock:
	Default + Copy + Clone + Send + Sync + Typed + FromReflect + GetTypeRegistration + 'static
{
}

impl<T> Clock for T where
	T: Default + Copy + Clone + Send + Sync + Typed + FromReflect + GetTypeRegistration + 'static
{
}

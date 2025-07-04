use bevy::{
	ecs::event::Event,
	prelude::{Deref, DerefMut},
};

#[derive(Event, Deref, DerefMut, Debug)]
pub struct RxNext<In>(pub In)
where
	In: 'static + Sync + Send;

#[derive(Event, Deref, DerefMut, Debug)]
pub struct RxError<InError>(pub InError)
where
	InError: 'static + Sync + Send;

#[derive(Event, Debug)]
pub struct RxComplete;

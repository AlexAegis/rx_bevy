use bevy::{
	ecs::event::Event,
	prelude::{Deref, DerefMut},
};

// TODO: Join these into a single enum if you don't want to spawn 3 of observer entities
#[derive(Event, Deref, DerefMut, Debug, Clone)]
pub struct RxNext<In>(pub In)
where
	In: 'static + Sync + Send;

#[derive(Event, Deref, DerefMut, Debug, Clone)]
pub struct RxError<InError>(pub InError)
where
	InError: 'static + Sync + Send;

#[derive(Event, Debug, Clone)]
pub struct RxComplete;

use bevy_ecs::{
	entity::{Entity, EntityCloner, EntityClonerBuilder},
	system::{EntityCommand, EntityCommands},
	world::EntityWorldMut,
};

pub fn clone_with_flush(
	target: Entity,
	config: impl FnOnce(&mut EntityClonerBuilder) + Send + Sync + 'static,
) -> impl EntityCommand {
	move |mut entity: EntityWorldMut| {
		entity.clone_flushed(target, config);
	}
}

pub trait EntityWorldMutFlushedCloneExt {
	fn clone_flushed(
		&mut self,
		target: Entity,
		config: impl FnOnce(&mut EntityClonerBuilder) + Send + Sync + 'static,
	) -> &mut Self;
}

impl<'w> EntityWorldMutFlushedCloneExt for EntityWorldMut<'w> {
	fn clone_flushed(
		&mut self,
		target: Entity,
		config: impl FnOnce(&mut EntityClonerBuilder) + Send + Sync + 'static,
	) -> &mut Self {
		self.location(); // to trigger self.assert_not_despawned();
		let id = self.id();

		unsafe {
			self.world_mut().flush();
		}

		let mut builder = EntityCloner::build(unsafe { self.world_mut() });

		config(&mut builder);

		builder.clone_entity(id, target);

		unsafe {
			self.world_mut().flush();
		}

		self.update_location();
		self
	}
}

pub trait EntityCloneFlushAndSpawnedWithExt {
	fn as_cloned_flushed_and_spawn_with<'n>(
		&mut self,
		target: Entity,
		config: impl FnOnce(&mut EntityClonerBuilder) + Send + Sync + 'static,
	) -> &mut Self;
}

impl<'w> EntityCloneFlushAndSpawnedWithExt for EntityCommands<'w> {
	fn as_cloned_flushed_and_spawn_with<'n>(
		&mut self,
		target: Entity,
		config: impl FnOnce(&mut EntityClonerBuilder) + Send + Sync + 'static,
	) -> &mut Self {
		self.queue(clone_with_flush(target, config));
		self
	}
}

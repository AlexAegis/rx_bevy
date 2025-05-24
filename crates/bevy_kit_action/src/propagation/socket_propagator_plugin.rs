use std::{any::TypeId, marker::PhantomData};

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Action, ActionSocket, ActionSystem, ActionSystemFor, Signal, SocketConnections};

use super::SocketConnectorSource;

#[derive_where(Default)]
pub struct SocketPropagatorPlugin<A: Action> {
	_phantom_data_action: PhantomData<A>,
}

impl<A: Action> Plugin for SocketPropagatorPlugin<A> {
	fn build(&self, app: &mut App) {
		app.add_event::<SignalPropagationEvent<A>>();

		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<A>::Propagate
				.after(ActionSystem::InputSocketWrite)
				.before(ActionSystem::PropagationDone),
		);

		app.add_systems(
			PreUpdate,
			kickstart_propagation::<A>.in_set(ActionSystemFor::<A>::Propagate),
		);

		// app.add_observer(react_to_propagation::<A>);
	}
}

//TODO: this chain of observables is very prone to infinite loops, track visited entities!
/// This internal event notifies others that this socket is ready to be
/// read from
#[derive(Event)]
pub(crate) struct SignalPropagationEvent<A: Action> {
	// pub(crate) action: A,
	// pub(crate) signal: A::Signal,
	// pub(crate) from_entity: Entity,
	pub(crate) visited: Vec<Entity>,
	_phantom_data_action: PhantomData<A>,
}

#[derive(Event)]
pub(crate) struct ErasedSignalPropagationEvent<S: Signal> {
	pub(crate) action_type: TypeId,
	pub(crate) signal: S, // pub(crate) visited: Vec<Entity>,
}

impl<A: Action> SignalPropagationEvent<A> {
	fn new(visited_entities: Vec<Entity>) -> Self {
		Self {
			//signal,
			// from_entity: entity,
			visited: visited_entities,
			_phantom_data_action: PhantomData,
		}
	}
}

/// Propagation is kick-started from sockets that have no sources themselves,
/// these sources are set "manually" meaning either by a built-in plugin
/// like keyboard events, or simply other sockets who's content is manually set
/// Sockets that have source connections would be overridden anyway so
/// manual sockets must not have sources of their own
fn kickstart_propagation<A: Action>(
	mut commands: Commands,
	connections_query: Query<
		(Entity, &SocketConnections<A>, &ActionSocket<A>),
		Without<SocketConnectorSource<A>>,
	>,
) {
	for (source_entity, source_entities_targets, action_socket) in connections_query.iter() {
		// commands.trigger_targets(
		// 	ErasedSignalPropagationEvent {
		// 		action_type: TypeId::of::<A>(),
		// 		signal: action_socket.get()
		// 	},
		// 	source_entities_targets.get_trigger_targets(),
		// );

		commands.trigger_targets(
			SignalPropagationEvent::<A>::new(vec![source_entity]),
			source_entities_targets.get_trigger_targets(),
		);
	}

	println!(
		"kickstart_propagation finished {:?}",
		std::any::type_name::<A>()
	);
}

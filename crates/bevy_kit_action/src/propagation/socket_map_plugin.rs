use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{
	Action, ActionSocket, ActionSocketPlugin, ActionSystem, ActionSystemFor, Clock,
	KeyboardInputSocket, SignalTransformer,
};

use super::{SocketConnector, SocketConnectorSource, SocketConnectorTarget};

/// Copies and transforms signals between sockets
#[derive_where(Default)]
pub struct SocketConnectorPlugin<
	C: Clock,
	FromAction,
	ToAction,
	Transformer, /* = SignalFromTransformer<
					 <FromAction as Action>::Signal,
					 <ToAction as Action>::Signal,
				 >*/
> where
	FromAction: Action,
	ToAction: Action,
	Transformer:
		SignalTransformer<C, InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>,
{
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
	_phantom_data_transformer: PhantomData<Transformer>,
	_phantom_data_clock: PhantomData<C>,
}

impl<C, FromAction, ToAction, Transformer> Plugin
	for SocketConnectorPlugin<C, FromAction, ToAction, Transformer>
where
	FromAction: Action,
	ToAction: Action,
	Transformer: SignalTransformer<C, InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>
		+ 'static
		+ Send
		+ Sync,
	C: Clock,
{
	fn build(&self, app: &mut App) {
		app.register_type::<SocketConnector<C, FromAction, ToAction, Transformer>>();

		// ActionSystem::InputSocketWrite
		// ActionSystemFor::<FromAction>::SocketWriteByConnector
		// ActionSystemFor::<FromAction>::SocketReadByConnector
		// ActionSystemFor::<ToAction>::SocketWriteByConnector
		// ActionSystemFor::<ToAction>::SocketReadByConnector
		// ActionSystem::Mapped

		if !app.is_plugin_added::<ActionSocketPlugin<ToAction>>() {
			app.add_plugins(ActionSocketPlugin::<ToAction>::default());
		}

		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<FromAction>::SocketWriteByConnector
				.after(ActionSystem::InputSocketWrite)
				.before(ActionSystem::Mapped),
		);

		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::SocketReadByConnector
				.after(ActionSystem::InputSocketWrite)
				.before(ActionSystem::Mapped),
		);

		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<FromAction>::SocketReadByConnector
				.before(ActionSystemFor::<ToAction>::SocketWriteByConnector)
				.after(ActionSystem::InputSocketWrite)
				.before(ActionSystem::Mapped),
		);

		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::SocketWriteByConnector
				.after(ActionSystemFor::<FromAction>::SocketReadByConnector)
				.after(ActionSystem::InputSocketWrite)
				.before(ActionSystem::Mapped),
		);

		// Make sure there is a resource of the converter to use it as the global fallback option
		// app.init_resource::<Transformer>();

		// Actions are triggered backwards compared to mapping
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Trigger.before(ActionSystemFor::<FromAction>::Trigger),
		);

		// The mapping system is running in the ToActions Map set as the action
		// it maps from is either created by a device, or manually entered
		app.add_systems(
			PreUpdate,
			from_socket_to_connector::<FromAction, ToAction, Transformer, C>
				.in_set(ActionSystemFor::<FromAction>::SocketReadByConnector),
		);

		app.add_systems(
			PreUpdate,
			from_connector_to_socket::<FromAction, ToAction, Transformer, C>
				.in_set(ActionSystemFor::<ToAction>::SocketWriteByConnector),
		);
	}
}

fn from_socket_to_connector<FromAction, ToAction, Transformer, C>(
	from_action_socket_query: Query<&ActionSocket<FromAction>>,
	mut to_action_socket_query: Query<&mut ActionSocket<ToAction>>,
	mut action_socket_query: Query<(
		Entity,
		&mut SocketConnector<C, FromAction, ToAction, Transformer>,
		Option<&SocketConnectorSource<FromAction>>,
		Option<&SocketConnectorTarget<ToAction>>,
	)>,
	// TODO: Should not be treated specially, as a global registered/root actionSocket of A
	keyboard_socket_query: Query<Entity, With<KeyboardInputSocket>>,
	time: Res<Time<C>>,
) where
	FromAction: Action,
	ToAction: Action,
	Transformer: SignalTransformer<C, InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>
		+ 'static
		+ Send
		+ Sync,
	C: Clock,
{
	// TODO: This should be automatically found by the connector
	let keyboard_entity_opt = keyboard_socket_query.single().ok();

	for (connector_entity, mut socket_connector, connector_source_opt, connector_target_opt) in
		action_socket_query.iter_mut()
	{
		let from_action_socket = connector_source_opt
			.and_then(|source| from_action_socket_query.get(source.entity()).ok())
			.or_else(|| from_action_socket_query.get(connector_entity).ok())
			.or_else(|| {
				// TODO: Impl a more generic way of collecting default source entities, a resource and a hashmap sounds okay, but what about gamepads, they must be per player
				keyboard_entity_opt
					.and_then(|keyboard_entity| from_action_socket_query.get(keyboard_entity).ok())
			});

		// This looks ugly but otherwise you'd get borrow problems
		let to_action_socket = {
			let exists_on_connector_target = connector_target_opt
				.map(|target| to_action_socket_query.contains(target.entity()))
				.unwrap_or(false);

			let entity = if exists_on_connector_target {
				connector_target_opt.unwrap().entity()
			} else {
				connector_entity
			};

			to_action_socket_query.get_mut(entity).ok()
		};

		let Some(from_action_socket) = from_action_socket else {
			error!("detached connector, missing source socket!");
			continue;
		};

		let Some(mut to_action_socket) = to_action_socket else {
			error!("detached connector, missing target socket!");
			continue;
		};

		for (from_action, from_action_signal_container) in from_action_socket.iter_containers() {
			let to_action = socket_connector.action_map.get(from_action).copied();

			if let Some(to_action) = to_action {
				let transformer_constructor = socket_connector
					.default_transformer_constructor
					.unwrap_or(Transformer::default);

				let transformer = socket_connector
					.signal_transformer_state
					.entry(to_action)
					.or_insert_with(transformer_constructor);

				transformer.write(
					&from_action_signal_container.signal,
					&time,
					&from_action_signal_container.last_frame_signal,
					to_action_socket.read_last_frame_signal_or_default(&to_action),
				);
			}
		}
	}
}

fn from_connector_to_socket<FromAction, ToAction, Transformer, C>(
	mut action_socket_query: Query<(
		&mut SocketConnector<C, FromAction, ToAction, Transformer>,
		&mut ActionSocket<ToAction>, // This shouldn't care about how it's stored as long as its mappable data
		                             // Option<&Transformer>,
	)>,
	_time: Res<Time<C>>,
) where
	FromAction: Action,
	ToAction: Action,
	Transformer: SignalTransformer<C, InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>
		+ 'static
		+ Send
		+ Sync,
	C: Clock,
{
	for (socket_connector, mut to_socket) in action_socket_query.iter_mut() {
		for (to_action, transformer) in socket_connector.signal_transformer_state.iter() {
			to_socket.write(to_action, transformer.read());
		}
	}
}

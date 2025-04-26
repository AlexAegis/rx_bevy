use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{
	Action, ActionSocket, ActionSocketPlugin, ActionSystem, ActionSystemFor, Clock,
	SignalTransformer,
};

use super::SocketConnector;

/// TODO: Maybe there could be a mutually exclusive way of setting up mapping between two actions, one is this HashMap based, and the other is just From<> impl based and would be faster and simpler but not configurable at runtime. Or it could be the default value for action pairs where it's implemented
/// Contains and executes activation between action contexts
#[derive(Reflect)]
#[derive_where(Default)]
pub struct SocketMapPlugin<
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
	#[reflect(ignore)]
	_phantom_data_from_action: PhantomData<FromAction>,
	#[reflect(ignore)]
	_phantom_data_to_action: PhantomData<ToAction>,
	#[reflect(ignore)]
	_phantom_data_transformer: PhantomData<Transformer>,
	#[reflect(ignore)]
	_phantom_data_clock: PhantomData<C>,
}

impl<C, FromAction, ToAction, Transformer> Plugin
	for SocketMapPlugin<C, FromAction, ToAction, Transformer>
where
	FromAction: Action,
	ToAction: Action,
	Transformer: SignalTransformer<C, InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>
		+ 'static
		+ Send
		+ Sync,
	Transformer::Buffer: 'static + Send + Sync,
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
	mut action_socket_query: Query<(
		&mut SocketConnector<C, FromAction, ToAction, Transformer>,
		&ActionSocket<FromAction>, // This shouldn't care about how it's stored as long as its mappable data
		&mut ActionSocket<ToAction>, // This shouldn't care about how it's stored as long as its mappable data
		                             // Option<&Transformer>,
	)>,
	time: Res<Time<C>>,
) where
	FromAction: Action,
	ToAction: Action,
	Transformer: SignalTransformer<C, InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>
		+ 'static
		+ Send
		+ Sync,
	Transformer::Buffer: 'static + Send + Sync,
	C: Clock,
{
	for (mut socket_connector, from_socket, mut to_socket) in action_socket_query.iter_mut() {
		for (from_action, from_action_signal_container) in from_socket.iter() {
			let to_action = socket_connector.action_map.get(from_action).copied();

			if let Some(to_action) = to_action {
				let transformer_constructor = socket_connector
					.default_transformer_constructor
					.unwrap_or(Transformer::default);

				let transformer = socket_connector
					.signal_transformer_state
					.entry(to_action)
					.or_insert_with(transformer_constructor);

				transformer.write_buffer(
					&from_action_signal_container.signal,
					&time,
					&from_action_signal_container.last_frame_signal,
					to_socket.read_last_frame_signal_or_default(&to_action),
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
	Transformer::Buffer: 'static + Send + Sync,
	C: Clock,
{
	for (socket_connector, mut to_socket) in action_socket_query.iter_mut() {
		for (to_action, transformer) in socket_connector.signal_transformer_state.iter() {
			to_socket.write(to_action, transformer.read());
		}
	}
}

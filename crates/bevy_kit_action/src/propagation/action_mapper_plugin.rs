use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{
	Action, ActionSocket, ActionSystem, ActionSystemFor, Clock, SignalFromTransformer,
	SignalTransformer,
};

use super::SocketConnector;

/// TODO: Maybe there could be a mutually exclusive way of setting up mapping between two actions, one is this HashMap based, and the other is just From<> impl based and would be faster and simpler but not configurable at runtime. Or it could be the default value for action pairs where it's implemented
/// Contains and executes activation between action contexts
#[derive(Resource)]
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
	_phantom_data_from_action: PhantomData<FromAction>,
	_phantom_data_to_action: PhantomData<ToAction>,
	_phantom_data_transformer: PhantomData<Transformer>,
	_phantom_data_clock: PhantomData<C>,
	default_transformer: Transformer,
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
	Transformer::InputBuffer: 'static + Send + Sync,
	C: Clock,
{
	fn build(&self, app: &mut App) {
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Map
				.after(ActionSystemFor::<FromAction>::Map)
				.after(ActionSystem::Input)
				.before(ActionSystem::Mapped),
		);

		// Make sure there is a resource of the converter to use it as the global fallback option
		// app.init_resource::<Transformer>();

		// Actions are triggered backwards compared to mapping
		// TODO: Does it matter? Which is better? This is kinda like bubbling. Should it be a crate feature?
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Trigger.before(ActionSystemFor::<FromAction>::Trigger),
		);

		// The mapping system is running in the ToActions Map set as the action
		// it maps from is either created by a device, or manually entered
		app.add_systems(
			PreUpdate,
			map_actions::<FromAction, ToAction, Transformer, C>
				.in_set(ActionSystemFor::<ToAction>::Map),
		);
	}
}

fn map_actions<FromAction, ToAction, Transformer, C>(
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
	Transformer::InputBuffer: 'static + Send + Sync,
	C: Clock,
{
	for (mut socket_connector, from_socket, mut to_socket) in action_socket_query.iter_mut() {
		for (from_action, from_action_signal) in from_socket.iter_signals() {
			let to_action = socket_connector.action_map.get(from_action).copied();

			let transformer_constructor = socket_connector
				.default_transformer_constructor
				.unwrap_or(Transformer::default);

			let transformer = socket_connector
				.signal_transformer_state
				.entry(*from_action)
				.or_insert_with(transformer_constructor);
			if let Some(to_action) = to_action {
				let converted_signal = transformer.transform_signal(from_action_signal, &time);
				to_socket.write(&to_action, converted_signal);
			}
		}
	}
}

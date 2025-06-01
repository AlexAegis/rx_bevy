use std::marker::PhantomData;

use bevy::{platform::collections::HashMap, prelude::*};
use derive_where::derive_where;
use smallvec::SmallVec;

use crate::{
	Action, ActionSocket, ActionSocketPlugin, ActionSystem, ActionSystemFor, Clock,
	KeyboardInputSocket, SignalTransformContext, SignalTransformer, SignalWriter, SocketAggregator,
	SocketConnections,
};

use super::{ConnectorTerminal, SocketConnector, SocketConnectorSource};

/// Copies and transforms signals between sockets
/// 1(A) -> 2(B) -> 3(C)
///     \-> 4(A) -> 5(B)
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
		SignalTransformer<InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>,
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
	Transformer: SignalTransformer<InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>
		+ 'static
		+ Send
		+ Sync,
	C: Clock,
{
	fn build(&self, app: &mut App) {
		#[cfg(feature = "reflect")]
		app.register_type::<SocketConnector<FromAction, ToAction, Transformer>>()
			.register_type::<ConnectorTerminal<ToAction>>();

		if !app.is_plugin_added::<ActionSocketPlugin<ToAction>>() {
			app.add_plugins(ActionSocketPlugin::<ToAction>::default());
		}

		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<FromAction>::TerminalWriteToSocket
				.after(ActionSystem::InputSocketWrite)
				.before(ActionSystem::PropagationDone),
		);

		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::SocketReadByConnectorWriteToTerminal
				.after(ActionSystem::InputSocketWrite)
				.before(ActionSystem::PropagationDone),
		);

		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<FromAction>::SocketReadByConnectorWriteToTerminal
				.before(ActionSystemFor::<ToAction>::TerminalWriteToSocket)
				.after(ActionSystem::InputSocketWrite)
				.before(ActionSystem::PropagationDone),
		);

		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::TerminalWriteToSocket
				.after(ActionSystemFor::<FromAction>::SocketReadByConnectorWriteToTerminal)
				.after(ActionSystem::InputSocketWrite)
				.before(ActionSystem::PropagationDone),
		);

		// Actions are triggered backwards compared to mapping
		app.configure_sets(
			PreUpdate,
			ActionSystemFor::<ToAction>::Trigger.before(ActionSystemFor::<FromAction>::Trigger),
		);

		// The mapping system is running in the ToActions Map set as the action
		// it maps from is either created by a device, or manually entered
		app.add_systems(
			PreUpdate,
			from_socket_through_connector_to_terminal::<FromAction, ToAction, Transformer, C>
				.in_set(ActionSystemFor::<FromAction>::SocketReadByConnectorWriteToTerminal),
		);

		app.add_systems(
			PreUpdate,
			from_terminal_to_socket::<ToAction>
				.in_set(ActionSystemFor::<ToAction>::TerminalWriteToSocket),
		);
	}
}

fn from_socket_through_connector_to_terminal<FromAction, ToAction, Transformer, C>(
	from_action_socket_query: Query<&ActionSocket<FromAction>>,
	mut to_action_socket_query: Query<&mut ActionSocket<ToAction>>,
	mut action_socket_query: Query<(
		Entity,
		&mut SocketConnector<FromAction, ToAction, Transformer>,
		&mut ConnectorTerminal<ToAction>,
		Option<&SocketAggregator<ToAction>>,
		Option<&SocketConnectorSource<FromAction>>,
	)>,
	// TODO: Should not be treated specially, as a global registered/root actionSocket of A
	keyboard_socket_query: Query<Entity, With<KeyboardInputSocket>>,
	time: Res<Time<C>>,
) where
	FromAction: Action,
	ToAction: Action,
	Transformer: SignalTransformer<InputSignal = FromAction::Signal, OutputSignal = ToAction::Signal>
		+ 'static
		+ Send
		+ Sync,
	C: Clock,
{
	// TODO: This should be automatically found by the connector
	let keyboard_entity_opt = keyboard_socket_query.single().ok();

	for (
		connector_entity,
		mut socket_connector,
		mut connector_terminal,
		aggregation_behavior,
		connector_source_opt,
	) in action_socket_query.iter_mut()
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
		let to_action_socket = { to_action_socket_query.get_mut(connector_entity).ok() };

		let Some(from_action_socket) = from_action_socket else {
			// trace!("detached connector, missing source socket!");
			continue;
		};

		let Some(mut to_action_socket) = to_action_socket else {
			// trace!("detached connector, missing target socket!");
			continue;
		};

		// TODO: Aggregating like this seems expensive, maybe it could be just an option on the target socket and skip it when not needed. or not even an option
		let mut signal_map = HashMap::<ToAction, SmallVec<[ToAction::Signal; 4]>>::new();

		for (from_action, from_action_signal_state) in from_action_socket.iter_containers() {
			let to_action = socket_connector.action_map.get(from_action).copied();

			if let Some(to_action) = to_action {
				let transformer_constructor = socket_connector
					.default_transformer_constructor
					.unwrap_or(Transformer::default);

				let transformer = socket_connector
					.signal_transformer_state
					.entry(to_action)
					.or_insert_with(transformer_constructor);

				let value = transformer.transform(
					&from_action_signal_state.signal,
					SignalTransformContext::<'_, C, FromAction::Signal> {
						time: &time,
						last_frame_input_signal: &from_action_signal_state.last_frame_signal,
						// last_frame_output_signal: to_action_socket
						// 	.read_last_frame_signal_or_default(&to_action),
					},
				);

				signal_map.entry(to_action).or_default().push(value);
			}
		}

		for (action, signals) in signal_map.iter() {
			connector_terminal.write_many(action, signals.iter().copied(), aggregation_behavior);
		}
	}
}

/// TODO:  The only "downside" is that this system will have more types, BUT proper aggregation is only possible like this, when it's possible to query all sources that wants to write here, otherwise we're stuck with another Vec buffer and re-aggregate on each write
///  Aren't [ConnectorTerminal]s just overhead? The result of a transformation
/// could be kept inside the connector component and read from that, after all
/// we already have a fully qualified plugin to fully specify the connectors
/// generics.
///
fn from_terminal_to_socket<A>(
	mut to_action_socket_query: Query<(
		Entity,
		Option<&SocketConnections<A>>,
		&mut ActionSocket<A>,
		Option<&SocketAggregator<A>>,
	)>,
	terminal_query: Query<&ConnectorTerminal<A>>,
) where
	A: Action,
{
	for (entity, connections, mut to_socket, accumulation_behavior) in
		to_action_socket_query.iter_mut()
	{
		let sources = Iterator::chain(
			std::iter::once(entity),
			connections
				.map(|c| c.get_trigger_targets())
				.into_iter()
				.flatten(),
		);

		let mut signal_map = HashMap::<A, SmallVec<[A::Signal; 4]>>::new();

		for source_terminal in terminal_query.iter_many(sources) {
			for (to_action, signal_accumulator) in source_terminal.iter() {
				signal_map
					.entry(*to_action)
					.or_default()
					.push(*signal_accumulator);
			}
		}

		for (action, signals) in signal_map.iter() {
			to_socket.write_many(action, signals.iter().copied(), accumulation_behavior);
		}
	}
}
